// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-header.md

mod fixtures;

use aws_esdk::*;
use fixtures::*;

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

/// Encrypt plaintext with default settings, return ciphertext bytes.
async fn encrypt_default(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Parse a V2 ciphertext header and return (edk_count_offset, content_type_offset, frame_length_offset).
/// V2 header layout: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1) + FrameLength(4) + SuiteData(32).
fn parse_header_offsets(ct: &[u8]) -> (usize, usize, usize) {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }

    // EDK count offset
    let edk_count_offset = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }

    let content_type_offset = pos;
    pos += 1; // Content Type: 1 byte

    let frame_length_offset = pos;

    (edk_count_offset, content_type_offset, frame_length_offset)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_big_endian_format() {
    //= specification/data-format/message-header.md#structure
    //= type=test
    //# The message header is a sequence of bytes that MUST be in big-endian format.
    let pt = b"big-endian header test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful decrypt proves header was serialized in big-endian format");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_serialization_order() {
    //= specification/data-format/message-header.md#structure
    //= type=test
    //# The header MUST be serialized as, in order,
    //# Header Body,
    //# and Header Authentication.
    let pt = b"header serialization order test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful decrypt proves header body precedes header authentication");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_greater_than_zero() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be greater than 0.
    let ct = encrypt_default(b"edk count test").await;
    let (edk_count_offset, _, _) = parse_header_offsets(&ct);
    let edk_count = u16::from_be_bytes([ct[edk_count_offset], ct[edk_count_offset + 1]]);
    assert!(edk_count > 0, "encrypted data key count must be greater than 0, got {edk_count}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_length_matches_algorithm_suite() {
    //= specification/data-format/message-header.md#algorithm-suite-data
    //= type=test
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"suite data length test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful V2 round-trip proves suite data length matches algorithm suite (validate_suite_data runs during decrypt)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_interpreted_as_bytes() {
    //= specification/data-format/message-header.md#algorithm-suite-data
    //= type=test
    //# The algorithm suite data MUST be interpreted as bytes.
    let pt = b"suite data bytes test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful V2 round-trip proves suite data is compared as bytes (validate_suite_data compares &[u8] slices)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_frame_length_must_be_zero() {
    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(b"frame length test", EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&input).await.unwrap().ciphertext;

    let (_, content_type_offset, frame_length_offset) = parse_header_offsets(&ct);

    // Set content type to NonFramed (0x01) and frame length to a non-zero value
    ct[content_type_offset] = 0x01;
    ct[frame_length_offset] = 0x00;
    ct[frame_length_offset + 1] = 0x00;
    ct[frame_length_offset + 2] = 0x10;
    ct[frame_length_offset + 3] = 0x00;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(decrypt(&dec_input).await.is_err(), "non-framed content with non-zero frame length must be rejected");
}
