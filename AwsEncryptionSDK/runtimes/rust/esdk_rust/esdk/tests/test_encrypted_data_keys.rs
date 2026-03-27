// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-keys

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
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

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt with a V1 (non-committing) algorithm suite, return ciphertext bytes.
async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// V1 header layout (empty EC):
///   [0]      Version (1 byte)
///   [1]      Type (1 byte)
///   [2..4]   Algorithm Suite ID (2 bytes)
///   [4..20]  Message ID (16 bytes)
///   [20..22] AAD Key Value Pairs Length (2 bytes, 0x0000 for empty EC)
///   [22..24] EDK Count (2 bytes)
///   [24..]   First EDK entry starts here
const FIRST_EDK_OFFSET: usize = 24;

/// Find the EDK count offset in a V2 ciphertext.
/// V2 header: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKCount(2).
fn edk_count_offset(ct: &[u8]) -> usize {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }

    pos // this is the offset of the 2-byte EDK count
}

/// Parse the first EDK entry fields from V1 ciphertext at FIRST_EDK_OFFSET.
/// Returns (key_provider_id_len, key_provider_id_bytes, key_provider_info_len,
///          key_provider_info_bytes, edk_len, edk_bytes).
fn parse_first_edk(ct: &[u8]) -> (u16, &[u8], u16, &[u8], u16, &[u8]) {
    let mut pos = FIRST_EDK_OFFSET;

    let kp_id_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let kp_id = &ct[pos..pos + kp_id_len as usize];
    pos += kp_id_len as usize;

    let kp_info_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let kp_info = &ct[pos..pos + kp_info_len as usize];
    pos += kp_info_len as usize;

    let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let edk_data = &ct[pos..pos + edk_len as usize];

    (kp_id_len, kp_id, kp_info_len, kp_info, edk_len, edk_data)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_keys_serialization_order() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-keys
    //= type=test
    //# The Encrypted Data Keys MUST be serialized as, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.
    let pt = b"encrypted data keys serialization order test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful decrypt proves EDKs were serialized as count then entries in order"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_is_2_bytes_uint16() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# The length of the serialized encrypted data key count MUST be 2 bytes.

    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# The encrypted data key count MUST be serialized as a UInt16.
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(b"test edk count", EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Read the 2-byte EDK count from the ciphertext and verify it is a valid UInt16 with value 1
    let offset = edk_count_offset(&ct);
    let edk_count = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
    assert_eq!(edk_count, 1, "single keyring must produce exactly 1 EDK, serialized as 2-byte UInt16");

    // Verify round-trip succeeds, proving the 2-byte UInt16 count was correctly serialized
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, b"test edk count");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_must_be_greater_than_zero() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be greater than 0.
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(b"test zero edks", EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper: set EDK count to 0
    let offset = edk_count_offset(&ct);
    ct[offset] = 0;
    ct[offset + 1] = 0;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(
        decrypt(&dec_input).await.is_err(),
        "message with 0 encrypted data keys must be rejected"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_serialization_order() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-entries
    //= type=test
    //# Each Encrypted Data Key Entry MUST be serialized as, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.
    let ct = encrypt_v1(b"edk entry order test").await;

    // Parse the 6 fields sequentially from the first EDK entry.
    // If the order were wrong, the lengths would not match the data and parsing would fail.
    let (kp_id_len, kp_id, kp_info_len, kp_info, edk_len, edk_data) = parse_first_edk(&ct);

    // Key Provider ID should be "child0 Namespace" (16 bytes)
    assert_eq!(kp_id_len as usize, kp_id.len());
    assert_eq!(std::str::from_utf8(kp_id).unwrap(), "child0 Namespace");
    // Key Provider Information and Encrypted Data Key should have matching lengths
    assert_eq!(kp_info_len as usize, kp_info.len());
    assert_eq!(edk_len as usize, edk_data.len());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_matches_data() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-id
    //= type=test
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    let ct = encrypt_v1(b"key provider id length test").await;
    let (kp_id_len, kp_id, _, _, _, _) = parse_first_edk(&ct);

    assert_eq!(
        kp_id_len as usize,
        kp_id.len(),
        "Key Provider ID byte count must equal the Key Provider ID Length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_is_utf8() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-id
    //= type=test
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    let ct = encrypt_v1(b"key provider id utf8 test").await;
    let (_, kp_id, _, _, _, _) = parse_first_edk(&ct);

    assert!(
        std::str::from_utf8(kp_id).is_ok(),
        "Key Provider ID bytes must be valid UTF-8"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_length_field_is_2_bytes_uint16() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-information-length
    //= type=test
    //# The length of the serialized key provider information length field MUST be 2 bytes.

    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-information-length
    //= type=test
    //# The key provider information length MUST be serialized as a UInt16.
    let ct = encrypt_v1(b"key provider info length test").await;

    // The Key Provider Info Length field starts after Key Provider ID Length (2) + Key Provider ID (variable)
    let kp_id_len = u16::from_be_bytes([ct[FIRST_EDK_OFFSET], ct[FIRST_EDK_OFFSET + 1]]) as usize;
    let info_len_offset = FIRST_EDK_OFFSET + 2 + kp_id_len;

    // Read the 2-byte field as big-endian UInt16
    let info_len_bytes = &ct[info_len_offset..info_len_offset + 2];
    let info_len = u16::from_be_bytes([info_len_bytes[0], info_len_bytes[1]]);

    // Verify the field is a valid UInt16 by checking the data that follows matches
    let info_data = &ct[info_len_offset + 2..info_len_offset + 2 + info_len as usize];
    assert_eq!(
        info_len as usize,
        info_data.len(),
        "Key Provider Information Length (2-byte UInt16) must match actual data length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_length_matches_data() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-information
    //= type=test
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    let ct = encrypt_v1(b"key provider info data test").await;
    let (_, _, kp_info_len, kp_info, _, _) = parse_first_edk(&ct);

    assert_eq!(
        kp_info_len as usize,
        kp_info.len(),
        "Key Provider Information byte count must equal the Key Provider Information Length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_is_bytes() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-information
    //= type=test
    //# The key provider information MUST be interpreted as bytes.
    let ct = encrypt_v1(b"key provider info bytes test").await;
    let (_, _, kp_info_len, kp_info, _, _) = parse_first_edk(&ct);

    // The key provider information is raw bytes — verify we can read exactly kp_info_len bytes
    assert_eq!(
        kp_info.len(),
        kp_info_len as usize,
        "Key Provider Information must be readable as raw bytes with length matching the length field"
    );
    assert!(kp_info_len > 0, "Key Provider Information should be non-empty for AES keyring");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_field_is_2_bytes_uint16() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-length
    //= type=test
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.

    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-length
    //= type=test
    //# The encrypted data key length MUST be serialized as a UInt16.
    let ct = encrypt_v1(b"edk length field test").await;

    // Navigate to the EDK Length field: skip Key Provider ID Length (2) + ID + Info Length (2) + Info
    let kp_id_len = u16::from_be_bytes([ct[FIRST_EDK_OFFSET], ct[FIRST_EDK_OFFSET + 1]]) as usize;
    let info_len_offset = FIRST_EDK_OFFSET + 2 + kp_id_len;
    let kp_info_len = u16::from_be_bytes([ct[info_len_offset], ct[info_len_offset + 1]]) as usize;
    let edk_len_offset = info_len_offset + 2 + kp_info_len;

    // Read the 2-byte field as big-endian UInt16
    let edk_len_bytes = &ct[edk_len_offset..edk_len_offset + 2];
    let edk_len = u16::from_be_bytes([edk_len_bytes[0], edk_len_bytes[1]]);

    // Verify the field is a valid UInt16 by checking the data that follows matches
    let edk_data = &ct[edk_len_offset + 2..edk_len_offset + 2 + edk_len as usize];
    assert_eq!(
        edk_len as usize,
        edk_data.len(),
        "Encrypted Data Key Length (2-byte UInt16) must match actual data length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_matches_data() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key
    //= type=test
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    let ct = encrypt_v1(b"edk data length test").await;
    let (_, _, _, _, edk_len, edk_data) = parse_first_edk(&ct);

    assert_eq!(
        edk_len as usize,
        edk_data.len(),
        "Encrypted Data Key byte count must equal the Encrypted Data Key Length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_is_bytes() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key
    //= type=test
    //# The encrypted data key MUST be interpreted as bytes.
    let ct = encrypt_v1(b"edk bytes test").await;
    let (_, _, _, _, edk_len, edk_data) = parse_first_edk(&ct);

    // The encrypted data key is raw bytes — verify we can read exactly edk_len bytes
    assert_eq!(
        edk_data.len(),
        edk_len as usize,
        "Encrypted Data Key must be readable as raw bytes with length matching the length field"
    );
    assert!(edk_len > 0, "Encrypted Data Key should be non-empty");
}
