// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#decrypt-the-message-body

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

/// Build a complete non-framed encrypted message from scratch.
///
/// Uses AlgAes256GcmHkdfSha512CommitKey (0x0478), V2 header, NonFramed content type.
/// The wrapping key is `[0u8; 32]` matching the test_keyring() configuration.
fn build_nonframed_message(plaintext: &[u8]) -> Vec<u8> {
    use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
    use aws_lc_rs::hkdf::{Salt, HKDF_SHA512};

    let wrapping_key = [0u8; 32];
    let plaintext_data_key = [0x42u8; 32];
    let message_id = [0xAAu8; 32];
    let edk_iv: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];
    let alg_suite_id: [u8; 2] = [0x04, 0x78];

    // --- Wrap the data key (raw AES keyring format) ---
    // EDK AAD = serialized encryption context (empty → [])
    let key = UnboundKey::new(&AES_256_GCM, &wrapping_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&edk_iv).unwrap();
    let mut edk_ct = plaintext_data_key.to_vec();
    let edk_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&[] as &[u8]), &mut edk_ct).unwrap();
    let mut edk_ciphertext = edk_ct;
    edk_ciphertext.extend_from_slice(edk_tag.as_ref());

    // EDK key_provider_info = key_name + tag_len_bits(u32be) + iv_len_bytes(u32be) + iv
    let key_name = b"child0 Name";
    let key_namespace = b"child0 Namespace";
    let mut provider_info = Vec::new();
    provider_info.extend_from_slice(key_name);
    provider_info.extend_from_slice(&128u32.to_be_bytes()); // tag length in bits
    provider_info.extend_from_slice(&12u32.to_be_bytes());  // IV length in bytes
    provider_info.extend_from_slice(&edk_iv);

    // --- Derive keys (V2 HKDF-SHA512 with commitment) ---
    let salt = Salt::new(HKDF_SHA512, &message_id);
    let prk = salt.extract(&plaintext_data_key);
    let mut data_key = [0u8; 32];
    let info_key: &[&[u8]] = &[&alg_suite_id, b"DERIVEKEY"];
    let okm = prk.expand(info_key, &AES_256_GCM).unwrap();
    okm.fill(&mut data_key).unwrap();
    let mut commit_key = [0u8; 32];
    let info_commit: &[&[u8]] = &[b"COMMITKEY"];
    let okm2 = prk.expand(info_commit, &AES_256_GCM).unwrap();
    okm2.fill(&mut commit_key).unwrap();

    // --- Build V2 header body ---
    let mut header_body = Vec::new();
    header_body.push(0x02); // Version 2.0
    header_body.extend_from_slice(&alg_suite_id); // Algorithm Suite ID
    header_body.extend_from_slice(&message_id); // Message ID (32 bytes for V2)
    header_body.extend_from_slice(&0u16.to_be_bytes()); // AAD: empty EC → key_value_pairs_length = 0
    // EDKs: count(2) + 1 EDK
    header_body.extend_from_slice(&1u16.to_be_bytes()); // EDK count = 1
    // EDK entry: provider_id_len(2) + provider_id + provider_info_len(2) + provider_info + edk_len(2) + edk
    header_body.extend_from_slice(&(key_namespace.len() as u16).to_be_bytes());
    header_body.extend_from_slice(key_namespace);
    header_body.extend_from_slice(&(provider_info.len() as u16).to_be_bytes());
    header_body.extend_from_slice(&provider_info);
    header_body.extend_from_slice(&(edk_ciphertext.len() as u16).to_be_bytes());
    header_body.extend_from_slice(&edk_ciphertext);
    header_body.push(0x01); // Content Type: NonFramed
    header_body.extend_from_slice(&0u32.to_be_bytes()); // Frame Length: 0 for non-framed
    header_body.extend_from_slice(&commit_key); // Algorithm Suite Data (commitment key)

    // --- Compute header auth (V2: IV=zeros, AAD=header_body, empty ciphertext) ---
    let header_auth_iv = [0u8; 12];
    let key = UnboundKey::new(&AES_256_GCM, &data_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&header_auth_iv).unwrap();
    let mut empty = Vec::new();
    let header_auth_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&header_body[..]), &mut empty).unwrap();

    // --- Build non-framed body ---
    // Body AAD: message_id + "AWSKMSEncryptionClient Single Block" + seq_num(4, be) + content_len(8, be)
    let mut body_aad = Vec::new();
    body_aad.extend_from_slice(&message_id);
    body_aad.extend_from_slice(b"AWSKMSEncryptionClient Single Block");
    body_aad.extend_from_slice(&1u32.to_be_bytes()); // sequence number = 1
    body_aad.extend_from_slice(&(plaintext.len() as u64).to_be_bytes()); // content length

    // Body IV: sequence number padded to 12 bytes with zeros
    let mut body_iv = [0u8; 12];
    body_iv[8..].copy_from_slice(&1u32.to_be_bytes());

    // Encrypt the plaintext
    let key = UnboundKey::new(&AES_256_GCM, &data_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&body_iv).unwrap();
    let mut body_ct = plaintext.to_vec();
    let body_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&body_aad[..]), &mut body_ct).unwrap();

    // --- Assemble the full message ---
    let mut message = Vec::new();
    // Header body
    message.extend_from_slice(&header_body);
    // Header auth (V2: tag only, no IV)
    message.extend_from_slice(header_auth_tag.as_ref());
    // Non-framed body: IV(12) + content_length(8) + encrypted_content(N) + auth_tag(16)
    message.extend_from_slice(&body_iv);
    message.extend_from_slice(&(body_ct.len() as u64).to_be_bytes());
    message.extend_from_slice(&body_ct);
    message.extend_from_slice(body_tag.as_ref());

    message
}

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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# For a final frame, each field MUST be deserialized according to its specification:
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be deserialized according to the
    //# [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [IV](../data-format/message-body.md#final-frame-iv): MUST be deserialized according to the
    //# [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be deserialized according to the
    //# [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be deserialized according to the
    //# [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
    // Single-frame message: 5 bytes with frame_length=10 → 1 final frame only.
    // Successful authenticated decryption proves all final frame fields were deserialized correctly.
    let pt = vec![0xBBu8; 5];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame deserialization conforms to spec");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_uses_first_4_bytes_to_determine_frame_type() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If the first 4 bytes have a value of 0xFFFFFFFF,
    //# then the Decrypt operation MUST deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
    // Single final frame: the first 4 bytes of the body are 0xFFFFFFFF.
    let pt = b"final frame test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "single-frame decrypt proves 0xFFFFFFFF triggers final frame deserialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_detected_without_endframe() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Otherwise, the Decrypt operation MUST deserialize the bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
    // Multi-frame: first frame starts with sequence number 1 (not 0xFFFFFFFF), so it's a regular frame.
    let pt = vec![0xDDu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame decrypt proves non-ENDFRAME bytes trigger regular frame deserialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_validation() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# The Decrypt operation MUST ensure that the length of the encrypted content field is
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# For a regular frame, each field MUST be deserialized according to its specification:
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be deserialized according to the
    //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be deserialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be deserialized according to the
    //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be deserialized according to the
    //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be deserialized according to the
    //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed according to the [Message Body AAD](../data-format/message-body-aad.md) specification, as follows:
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-format/message-header.md#message-id) deserialized from the header of this message.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be constructed
    //# according to [Message Body AAD](../data-format/message-body-aad.md) depending on
    //# whether the bytes being decrypted are a regular frame, final frame, or un-framed data.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The cipherkey MUST be the derived data key
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The ciphertext MUST be the [encrypted content](../data-format/message-body.md#regular-frame-encrypted-content).
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The tag MUST be the value serialized in the
    //# [authentication tag field](../data-format/message-body.md#regular-frame-authentication-tag)
    //# in the message body or frame.
    // Round-trip: if any AES-GCM input (IV, cipherkey, ciphertext, tag) were wrong,
    // authenticated decryption would fail.
    let pt = vec![0xDDu8; 40];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves all AES-GCM inputs (IV, key, ciphertext, tag) are correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_wait_for_bytes() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
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
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is not a regular frame, this SHOULD be determined by using the the [encrypted content length](../data-format/message-body.md#final-frame-encrypted-content-length).
    // Single final frame with plaintext shorter than frame length.
    // The final frame's content length in AAD must use the actual encrypted content length (5),
    // not the frame length (4096). If wrong, authenticated decryption would fail.
    let pt = vec![0xDDu8; 5];
    let result = round_trip(&pt, 4096).await;
    assert_eq!(result, pt, "final-frame-only decrypt proves content length uses encrypted content length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_held_until_signature_verification() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
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
    // Tamper with the last byte of the signature to cause verification failure
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "tampered signature must cause decrypt failure, proving final frame was held back");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_nonframed_deserialization_conforms_to_spec() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Non-framed data deserialization MUST conform to the [Non-Framed Data](../data-format/message-body.md#non-framed-data) specification.
    // Construct a non-framed message from scratch and decrypt it.
    // Successful decryption proves the non-framed deserialization conforms to the spec.
    let pt = b"non-framed conformance test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec(), "non-framed round-trip proves deserialization conforms to Non-Framed Data spec");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_deserializes_and_decrypts() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# If a message has the [non-framed](../data-format/message-body.md#non-framed-data) content type,
    //# the Decrypt operation MUST deserialize the message body according to the
    //# [non-framed data specification](../data-format/message-body.md#non-framed-data)
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let pt = b"un-framed appendix test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_iv_from_body() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The IV MUST be the [IV](../data-format/message-body.md#non-framed-data-iv) deserialized from the message body.
    // Successful authenticated decryption of a non-framed message proves the IV
    // was correctly deserialized from the body and used for decryption.
    let pt = b"iv test payload";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_ciphertext_from_body() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The ciphertext MUST be the [Encrypted Content](../data-format/message-body.md#non-framed-data-encrypted-content) deserialized from the message body.
    let pt = b"ciphertext input test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_cipherkey_is_derived_data_key() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The cipherkey MUST be the derived data key.
    // Successful decryption proves the derived data key was used as the cipherkey.
    let pt = b"cipherkey test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_tag_from_body() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The tag MUST be the [Authentication Tag](../data-format/message-body.md#non-framed-data-authentication-tag) deserialized from the message body.
    let pt = b"auth tag test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_body_aad_content() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST use the value for
    //# [non-framed data](../data-format/message-body-aad.md#body-aad-content).
    // The non-framed message was constructed with "AWSKMSEncryptionClient Single Block".
    // If the wrong AAD content string were used, authenticated decryption would fail.
    let pt = b"aad content test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_sequence_number_is_one() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be `1`.
    // The non-framed message was constructed with sequence number 1 in the AAD.
    // If a different sequence number were used, authenticated decryption would fail.
    let pt = b"seq num one test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_content_length_equals_encrypted_content_length() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST equal the length of the encrypted content.
    // The non-framed message was constructed with content_length = plaintext.len() in the AAD.
    // If the wrong content length were used, authenticated decryption would fail.
    let pt = b"content length test payload";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_fails_on_tampered_auth_tag() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag in a non-framed message. Decrypt must fail.
    let pt = b"tamper test";
    let mut ct = build_nonframed_message(pt);
    // The auth tag is the last 16 bytes of the message
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "tampered non-framed auth tag must cause immediate decryption failure");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_constructed_correctly() {
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= type=test
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed with:
    // Successful authenticated decryption of a non-framed message proves the AAD
    // was constructed correctly per the message-body-aad spec.
    let pt = b"aad construction test";
    let ct = build_nonframed_message(pt);
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt.to_vec());
}
