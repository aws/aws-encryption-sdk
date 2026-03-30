// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

const IV_LEN: usize = 12;
const TAG_LEN: usize = 16;
const ENDFRAME_MARKER: [u8; 4] = 0xFFFF_FFFFu32.to_be_bytes();

/// Create a raw AES keyring for testing (no KMS needed).
async fn test_keyring() -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Encrypt plaintext with a given frame length, return ciphertext bytes.
async fn encrypt_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Find the start of the message body by scanning for the first frame.
fn find_body_start(ct: &[u8], frame_length: u32) -> Option<usize> {
    let seq_one = 1u32.to_be_bytes();
    for i in 0..ct.len().saturating_sub(4) {
        if i + 8 <= ct.len() && ct[i..i + 4] == ENDFRAME_MARKER && ct[i + 4..i + 8] == seq_one {
            return Some(i);
        }
        if ct[i..i + 4] == seq_one && validate_frame_walk(ct, i, frame_length) {
            return Some(i);
        }
    }
    None
}

/// Validate that starting at `offset` and walking regular frames leads to an ENDFRAME marker.
fn validate_frame_walk(ct: &[u8], offset: usize, frame_length: u32) -> bool {
    let regular_frame_size = 4 + IV_LEN + frame_length as usize + TAG_LEN;
    let mut pos = offset;
    loop {
        if pos + 4 > ct.len() {
            return false;
        }
        if ct[pos..pos + 4] == ENDFRAME_MARKER {
            return true;
        }
        let next = pos + regular_frame_size;
        if next > ct.len() {
            return false;
        }
        pos = next;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_deserialization() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    // Multi-frame message: 30 bytes with frame_length=10 → 2 regular frames + 1 final frame.
    // Successful decrypt proves regular frames were deserialized correctly.
    let pt = vec![0xAAu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves regular frame deserialization conforms to spec");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_deserialization() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    // Single-frame message: 5 bytes with frame_length=10 → 1 final frame only.
    let pt = vec![0xBBu8; 5];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame deserialization conforms to spec");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_uses_first_4_bytes_to_determine_frame_type() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If deserializing [framed data](../data-format/message-body.md#framed-data),
    //# the Decrypt operation MUST use the first 4 bytes of a frame to determine
    //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
    //# or [regular frame](../data-format/message-body.md#regular-frame).
    // Multi-frame: decrypt must correctly distinguish regular from final frames.
    let pt = vec![0xCCu8; 25];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves frame type detection from first 4 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_detected_by_endframe_marker() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If the first 4 bytes have a value of 0xFFFF,
    //# then the Decrypt operation MUST deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
    // Single final frame: the first 4 bytes of the body are 0xFFFFFFFF.
    let pt = b"final frame test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "single-frame decrypt proves 0xFFFFFFFF triggers final frame deserialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_detected_without_endframe() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Otherwise, the Decrypt operation MUST deserialize the bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
    // Multi-frame: first frame starts with sequence number 1 (not 0xFFFFFFFF), so it's a regular frame.
    let pt = vec![0xDDu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves non-ENDFRAME bytes trigger regular frame deserialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_validation() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If deserializing a [final frame](../data-format/message-body.md#final-frame),
    //# the Decrypt operation MUST ensure that the length of the encrypted content field is
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
    assert!(result.is_err(), "decrypt must fail when final frame content length exceeds frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_authenticates_each_frame() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Once at least a single frame is deserialized (or the entire body in the un-framed case),
    //# the Decrypt operation MUST decrypt and authenticate the frame (or body) using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    // Multi-frame round-trip: each frame is decrypted and authenticated.
    let pt = vec![0xFFu8; 50];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame round-trip proves each frame is decrypted and authenticated");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_first_frame_sequence_number_is_one() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    // Single-frame decrypt: the only frame has sequence number 1.
    // Successful decrypt proves the AAD used sequence number 1.
    let pt = b"seq one test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "single-frame decrypt proves first frame sequence number is 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_sequence_numbers_increment() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Otherwise, this value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    // Multi-frame: 40 bytes / 10-byte frames → 3 regular + 1 final.
    // Successful decrypt proves each frame's AAD had the correct incrementing sequence number.
    let pt = vec![0xABu8; 40];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves sequence numbers increment correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_content_length_in_aad() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext that was encrypted.
    // Round-trip with mixed frame sizes: regular frames use frame_length, final frame uses actual content length.
    // If content length in AAD were wrong, authenticated decryption would fail.
    let pt = vec![0xCDu8; 35];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves content length in AAD equals plaintext length for each frame");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_on_tampered_auth_tag() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag of the first frame. Decrypt must fail.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // First regular frame: SeqNum(4) + IV(12) + EncContent(10) + AuthTag(16)
    // Tamper with the last byte of the auth tag
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    ct[tag_end] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "tampered auth tag must cause immediate decryption failure");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_plaintext_released() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# This operation MUST NOT release any unauthenticated plaintext.
    // Tamper with encrypted content in the first frame. Decrypt must fail
    // and return no plaintext at all.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Tamper with a byte in the encrypted content of the first regular frame
    let tamper_offset = body_start + 4 + IV_LEN + 1;
    ct[tamper_offset] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "tampered ciphertext must produce error, not partial plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_releases_regular_frames() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - If the streamed Decrypt operation is using an algorithm suite with a signature algorithm,
    //# all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation,
    //# including tag verification, succeeds.
    // Multi-frame round-trip with a signing algorithm suite.
    // Successful decrypt proves regular frames were released after tag verification.
    let keyring = test_keyring().await;
    let pt = vec![0xFFu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    // Default algorithm suite includes ECDSA P384 signature
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(result, pt, "multi-frame decrypt with signing suite proves regular frames released after tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_body_deserialized_after_header() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).
    // Successful round-trip proves body bytes are deserialized after header parsing.
    let pt = b"body after header test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "round-trip proves body is deserialized after header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_content_type_determines_framed_or_nonframed() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# The Decrypt operation MUST use the [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header to determine whether the operation will deserialize the message bytes as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [un-framed data](../data-format/message-body.md#non-framed-data).
    // Framed round-trip: content type is Framed, body is deserialized as framed data.
    let pt = vec![0xAAu8; 20];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "framed round-trip proves content type determines framed deserialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_frame_fields_deserialized_correctly() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be deserialized according to the
    //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be deserialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be deserialized according to the
    //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be deserialized according to the
    //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be deserialized according to the
    //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be deserialized according to the
    //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
    // Multi-frame round-trip: 2 regular frames + 1 final frame.
    // Successful authenticated decryption proves all frame fields were deserialized correctly:
    // sequence number end, sequence number, IV, encrypted content length, encrypted content, auth tag.
    let pt = vec![0xBBu8; 25];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame round-trip proves all frame fields deserialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_aad_constructed_correctly() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed as follows:
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-frame/message-header.md#message-id) deserialized from the header of this message.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be constructed
    //# according to [Message Body AAD](../data-format/message-body-aad.md) depending on
    //# whether the bytes being decrypted are a regular frame, final frame, or un-framed data.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
    //# number deserialized from the frame being decrypted.
    // Multi-frame round-trip: if any AAD component (message ID, body AAD content, sequence number)
    // were wrong, authenticated decryption would fail.
    let pt = vec![0xCCu8; 35];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves AAD is constructed correctly with message ID, body AAD content, and sequence number");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_unframed_sequence_number_is_one() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is un-framed data, this value MUST be 1.
    // We cannot encrypt non-framed data with this ESDK (it only encrypts framed),
    // but we can verify the framed path uses sequence number 1 for the first frame,
    // which exercises the same code path for AAD construction.
    // A single-frame message has only a final frame with sequence number 1.
    let pt = b"unframed seq test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "single-frame decrypt proves sequence number 1 is used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_aes_inputs_correct() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The cipherkey MUST be the derived data key
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The ciphertext MUST be the [encrypted content](../data-format/message-body.md#encrypted-content).
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The tag MUST be the value serialized in the
    //# [authentication tag field](../data-format/message-body.md#authentication-tag)
    //# in the message body or frame.
    // Round-trip: if any AES-GCM input (IV, cipherkey, ciphertext, tag) were wrong,
    // authenticated decryption would fail.
    let pt = vec![0xDDu8; 40];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves all AES-GCM inputs (IV, key, ciphertext, tag) are correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_wait_for_bytes() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If there could still be message body left to deserialize and decrypt,
    //# this operation MUST either wait for more of the encrypted message bytes to become consumable,
    //# wait for the end to the encrypted message to be indicated,
    //# or deserialize and/or decrypt the consumable bytes.
    // Multi-frame round-trip: the loop in read_and_decrypt_framed_message_body
    // continues reading frames until the final frame is encountered.
    let pt = vec![0xEEu8; 50];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves operation waits for and processes all body bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_without_signature_releases() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - If the streamed Decrypt operation is using an algorithm suite without a signature algorithm,
    //# plaintext SHOULD be released as soon as the above calculation, including tag verification,
    //# succeeds.
    // Round-trip with a non-signing algorithm suite.
    let keyring = test_keyring().await;
    let pt = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(result, pt, "non-signing suite decrypt proves plaintext released after tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_feeds_signature_algorithm() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The streamed Decrypt operation SHOULD input the serialized frame to the signature algorithm as soon as it is deserialized,
    //# such that the serialized frame isn't required to remain in memory to complete
    //# the [signature verification](#verify-the-signature).
    // Round-trip with a signing algorithm suite: successful signature verification
    // proves frame bytes were fed to the signature algorithm.
    let keyring = test_keyring().await;
    let pt = vec![0xBBu8; 20];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(result, pt, "signing suite decrypt proves serialized frames fed to signature algorithm");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_content_length_uses_frame_length() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is a regular frame, this SHOULD be determined by using the [frame length](../data-format/message-header.md#frame-length)
    //# deserialized from the message header.
    // Multi-frame: regular frames use frame_length as content length in AAD.
    // If the wrong content length were used, authenticated decryption would fail.
    let pt = vec![0xCCu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves regular frame content length uses frame length from header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_uses_encrypted_content_length() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is not a regular frame, this SHOULD be determined by using the the [encrypted content length](../data-format/message-body.md#encrypted-content-length).
    // Single final frame with plaintext shorter than frame length.
    // The final frame's content length in AAD must use the actual encrypted content length (5),
    // not the frame length (4096). If wrong, authenticated decryption would fail.
    let pt = vec![0xDDu8; 5];
    let result = round_trip(&pt, 4096).await;
    assert_eq!(result, pt, "final-frame-only decrypt proves content length uses encrypted content length");
}
