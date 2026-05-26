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
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# MUST ensure that the length of the encrypted content field is
    //# less than or equal to the frame length deserialized in the message header.
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
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_on_tampered_auth_tag() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag of the first frame. Decrypt must fail.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Baseline: untampered ciphertext decrypts successfully.
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "baseline must pass");
    // First regular frame: SeqNum(4) + IV(12) + EncContent(10) + AuthTag(16)
    // Tamper with the last byte of the auth tag
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    ct[tag_end] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered auth tag must cause immediate decryption failure");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_plaintext_released() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# This operation MUST NOT release any unauthenticated plaintext.
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
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_unframed_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Decrypt output of the external V2 nonframed vector matches the expected plaintext, which can only happen if the decryptor used sequence number 1 in its AAD reconstruction.
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
    //# - The ciphertext MUST be the [Encrypted Content](../data-format/message-body.md#nonframed-data-encrypted-content) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_cipherkey_is_derived_data_key() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The cipherkey MUST be the derived data key.
    // Successful decryption proves the derived data key was used as the cipherkey.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_tag_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The tag MUST be the [Authentication Tag](../data-format/message-body.md#nonframed-data-authentication-tag) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_body_aad_content() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
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
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be `1`.
    // External V2 nonframed vector was produced with sequence number 1 in the AAD.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_content_length_equals_encrypted_content_length() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
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
    //# The first frame's sequence number MUST be 1.
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
    //# Sequence numbers MUST be in order and MUST start at 1.
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
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}
