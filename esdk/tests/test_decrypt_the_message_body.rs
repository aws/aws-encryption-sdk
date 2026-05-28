// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#decrypt-the-message-body

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_validation() {
    // Encrypt a message, then tamper with the final frame's content length field
    // to exceed the frame length. Decrypt must fail.
    let pt = vec![0xEEu8; 5];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    // Find the ENDFRAME marker, then the content length is at offset +20 (ENDFRAME(4)+SeqNum(4)+IV(12))
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == ENDFRAME_MARKER {
            // Set content length to frame_length + 1 = 11 (exceeds frame_length=10)
            let bad_len = 11u32.to_be_bytes();
            ct[i + 20..i + 24].copy_from_slice(&bad_len);
            break;
        }
    }
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when final frame content length exceeds frame length");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# MUST ensure that the length of the encrypted content field is
    //# less than or equal to the frame length deserialized in the message header.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered content length > frame_length causes error, proving it's checked
    //# If this is a final frame, this MUST be determined by using the [final frame encrypted content length](../data-format/message-body.md#final-frame-encrypted-content-length).
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_on_tampered_auth_tag() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered auth tag → CryptographicError proves AEAD check runs
    //# If this decryption fails, this operation MUST immediately halt and fail.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "baseline must pass");
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    ct[tag_end] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered auth tag must fail");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_fields_deserialized_from_wire() {
    // Directly verify each frame field is present at the correct wire offset.
    // 30 bytes / frame_length=10 → 2 regular + 1 final = 3 frames.
    let pt = vec![0xABu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames.len(), 3, "expected 3 frames (2 regular + 1 final)");

    // Regular frame 1: verify deserialized fields on wire
    let f = &frames[0];
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire seq_num at expected offset proves it was deserialized
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    assert_eq!(f.seq_num, 1, "frame 1 seq_num must be 1");
    assert_eq!(f.seq_num_bytes.len(), 4, "seq_num is 4 bytes on wire");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire IV at expected offset proves it was deserialized
    //# - MUST deserialize the [IV](../data-format/message-body.md#regular-frame-iv).
    assert_eq!(f.iv.len(), IV_LEN, "IV is 12 bytes on wire");
    assert_eq!(f.iv_offset, f.seq_num_offset + 4, "IV follows seq_num");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire content at expected offset proves it was deserialized
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
    assert_eq!(f.content.len(), 10, "regular frame content is frame_length bytes");
    assert_eq!(f.content_offset, f.iv_offset + IV_LEN, "content follows IV");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire tag at expected offset proves it was deserialized
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
    assert_eq!(f.tag.len(), TAG_LEN, "auth tag is 16 bytes on wire");
    assert_eq!(f.tag_offset, f.content_offset + 10, "tag follows content");

    // Final frame: verify content_length field
    let ff = &frames[2];
    assert!(ff.is_final, "frame 3 must be final");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire content_length field in final frame proves it was deserialized
    //# - MUST deserialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    assert_eq!(ff.content_length_bytes.unwrap().len(), 4, "content_length is 4 bytes");
    assert_eq!(ff.content_length, 10, "final frame content = remaining 10 bytes");

    // Round-trip corroboration: all fields consumed correctly
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_plaintext_released() {
    // Tamper with encrypted content in the first frame. Decrypt must fail
    // and return no plaintext at all.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Baseline: untampered ciphertext decrypts successfully.
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "baseline must pass");
    // Tamper with a byte in the encrypted content of the first regular frame
    let tamper_offset = body_start + 4 + IV_LEN + 1;
    ct[tamper_offset] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered ciphertext must produce error, not partial plaintext");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered body returns Err; no DecryptOutput means no plaintext released
    //# This operation MUST NOT release any unauthenticated plaintext.
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_unframed_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=External V2 nonframed vector succeeds only if AAD sequence number is 1
    //# If this is nonframed data, this value MUST be 1.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — AAD sequence number is not 1"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_nonframed_content_length_determines_aad() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=External vector decrypts; wrong content length would fail AES-GCM
    //# If this is nonframed data, this MUST be determined by using the [nonframed data encrypted content length](../data-format/message-body.md#nonframed-data-encrypted-content-length).
    // Successful decryption of the external V2 nonframed vector implies the decryptor's AAD content length matched what the external producer used — which, for that vector, is the nonframed data encrypted content length.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — AAD content length did not come from the nonframed encrypted content length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_held_until_signature_verification() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered signature → Err; proves final frame plaintext was held back
    //# Any plaintext decrypted from [nonframed data](../data-format/message-body.md#nonframed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.
    // Encrypt with a signing algorithm suite, then tamper with the signature.
    // Decrypt must fail, proving the final frame plaintext was held back
    // pending signature verification and never released.
    let keyring = test_keyring().await;
    let pt = vec![0xABu8; 16];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(4096).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;
    // Baseline: untampered ciphertext decrypts successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline must pass before tamper");
    // Tamper with the last byte of the signature to cause verification failure
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered signature must cause decrypt failure, proving final frame was held back");
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_nonframed_deserialization_conforms_to_spec() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector from aws-encryption-sdk-python decrypts; proves format conformance
    //# Nonframed data deserialization MUST conform to the [Nonframed Data](../data-format/message-body.md#nonframed-data) specification.
    // Successful decryption of the external V2 nonframed vector (produced by
    // aws-encryption-sdk-python 2.0.0) proves our nonframed deserialization
    // conforms to the spec.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — nonframed deserialization does not conform to spec"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_deserializes_and_decrypts() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External nonframed vector decrypts; proves nonframed path works
    //# If a message has the [nonframed](../data-format/message-body.md#nonframed-data) content type,
    //# the Decrypt operation MUST deserialize the message body according to the
    //# [nonframed data specification](../data-format/message-body.md#nonframed-data)
    //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_iv_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong IV would fail AES-GCM
    //# - The IV MUST be the [IV](../data-format/message-body.md#nonframed-data-iv) deserialized from the message body.
    // Successful authenticated decryption of the external V2 nonframed vector
    // proves the IV was correctly deserialized from the body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_ciphertext_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong ciphertext would fail
    //# - The ciphertext MUST be the [Encrypted Content](../data-format/message-body.md#nonframed-data-encrypted-content) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_cipherkey_is_derived_data_key() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector decrypts; wrong key would fail AES-GCM
    //# - The cipherkey MUST be the derived data key.
    // Successful decryption proves the derived data key was used as the cipherkey.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_tag_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong tag would fail AES-GCM
    //# - The tag MUST be the [Authentication Tag](../data-format/message-body.md#nonframed-data-authentication-tag) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_body_aad_content() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong AAD content would fail
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST use the value for
    //# [nonframed data](../data-format/message-body-aad.md#body-aad-content).
    // External V2 nonframed vector was produced with the spec-required
    // "AWSKMSEncryptionClient Single Block" body AAD content. If our
    // decryptor reconstructed a different content value, AES-GCM auth
    // would fail and decryption would not return the expected plaintext.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong seq num in AAD would fail
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be `1`.
    // External V2 nonframed vector was produced with sequence number 1 in the AAD.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_content_length_equals_encrypted_content_length() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong content length would fail
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST equal the length of the plaintext.
    // External V2 nonframed vector's AAD content_length equals its plaintext length.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_fails_on_tampered_auth_tag() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag in the external V2 nonframed vector. Decrypt must fail.
    let mut ct = EXTERNAL_V2_NONFRAMED_CT.to_vec();
    // Baseline: untampered vector decrypts successfully.
    let baseline = try_decrypt_external_nonframed(Version::V2, &ct).await;
    assert!(baseline.is_ok(), "baseline external vector must decrypt");
    // The auth tag is the last 16 bytes of the message
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let result = try_decrypt_external_nonframed(Version::V2, &ct).await;
    let err = result.expect_err("tampered nonframed auth tag must cause immediate decryption failure");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_constructed_correctly() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector auth succeeds; wrong AAD would fail AES-GCM
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed with:
    // Successful authenticated decryption of the external V2 nonframed vector
    // proves the AAD was constructed correctly per the message-body-aad spec.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_value_is_0xffffffff() {
    let pt = vec![0xBBu8; 5];
    let ct = encrypt_with_frame_length(&pt, 4096).await;
    let frames = parse_frames(&ct, 4096);
    assert_eq!(frames.len(), 1, "single final frame expected");
    assert!(frames[0].4, "frame must be a final frame");
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Final frame starts with 0xFFFFFFFF on wire; decrypt validates it
    //# The value MUST be `0xFFFFFFFF`.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves ENDFRAME marker was deserialized
    //# - MUST deserialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves seq num was deserialized
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves IV was deserialized
    //# - MUST deserialize the [IV](../data-format/message-body.md#final-frame-iv).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves content length was deserialized
    //# - MUST deserialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves encrypted content was deserialized
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful final frame decrypt proves auth tag was deserialized and checked
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_content_length_from_encrypted_content_length() {
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=External nonframed vector proves content length is read from the wire field
    //# If this is nonframed data, this MUST be determined by using the [nonframed data encrypted content length](../data-format/message-body.md#nonframed-data-encrypted-content-length).
    assert_eq!(
        body.encrypted_content_length as usize,
        EXTERNAL_V2_NONFRAMED_PT.len(),
        "encrypted content length field must equal plaintext length"
    );
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_first_frame_sequence_number_is_one() {
    // Parse the on-wire ciphertext and verify the first frame's sequence number is 1.
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert!(frames.len() >= 2, "need at least 2 frames");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire first frame seq_num == 1 proves requirement
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Wire parse extracts seq_num from first 4 bytes, proving deserialization
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful multi-frame decrypt proves body bytes follow header
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful decrypt of all frames proves bytes were consumed as available
    //# If there could still be message body left to deserialize and decrypt,
    //# this operation MUST either wait for more of the encrypted message bytes to become consumable,
    //# wait for the end to the encrypted message to be indicated,
    //# or deserialize and/or decrypt the consumable bytes.
    assert_eq!(frames[0].seq_num, 1, "first frame sequence number must be 1 on the wire");
    // Round-trip corroboration
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_sequence_numbers_increment() {
    // Parse 5 frames and verify sequence numbers are 1, 2, 3, 4, 5.
    // 50 bytes at frame_length=10 → 4 regular + 1 final = 5 frames.
    let pt = vec![0xBBu8; 50];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire seq_nums 1..5 prove monotonic increment across frames
    //# Otherwise, this value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    assert_eq!(frames.len(), 5, "50 bytes / 10-byte frames = 4 regular + 1 final");
    for (i, frame) in frames.iter().enumerate() {
        assert_eq!(
            frame.seq_num,
            (i + 1) as u32,
            "frame {i}: sequence number must be {} on the wire",
            i + 1
        );
    }
    // Round-trip corroboration
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful 5-frame decrypt proves content type dispatches to framed data
    //# The Decrypt operation MUST use the [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header to determine whether the operation will deserialize the message bytes as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [nonframed data](../data-format/message-body.md#nonframed-data).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful 5-frame decrypt proves first 4 bytes determine frame type
    //# If deserializing [framed data](../data-format/message-body.md#framed-data),
    //# the Decrypt operation MUST use the first 4 bytes of a frame to determine
    //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
    //# or [regular frame](../data-format/message-body.md#regular-frame).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=5-frame decrypt proves first 4 bytes of each frame are inspected
    //# The Decrypt operation MUST inspect the first 4 bytes of each frame.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Final frame detected and deserialized per final frame spec
    //# If the first 4 bytes have a value of 0xFFFFFFFF,
    //# the Decrypt operation MUST treat them as the [Sequence Number End](../data-format/message-body.md#sequence-number-end)
    //# and deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Regular frames detected and deserialized per regular frame spec
    //# Otherwise, the Decrypt operation MUST treat them as the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number)
    //# and deserialize the following bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=4 regular frames successfully decrypted
    //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=4 regular frames deserialized per field spec
    //# For a regular frame, each field MUST be deserialized according to its specification:
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Final frame successfully decrypted
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Final frame deserialized per field spec
    //# For a final frame, each field MUST be deserialized according to its specification:
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}
