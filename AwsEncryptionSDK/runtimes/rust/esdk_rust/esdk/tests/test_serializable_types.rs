// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-header.md
//! Covers: #key-provider-id-length, #key-provider-information-length, #encrypted-data-key-length

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

/// Encrypt with a V1 (non-committing) algorithm suite, return ciphertext bytes.
async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Find the offset of the first EDK entry in a V1 ciphertext with empty encryption context.
/// V1 header layout (empty EC):
///   [0]      Version (1 byte)
///   [1]      Type (1 byte)
///   [2..4]   Algorithm Suite ID (2 bytes)
///   [4..20]  Message ID (16 bytes)
///   [20..22] AAD Key Value Pairs Length (2 bytes, 0x0000 for empty EC)
///   [22..24] EDK Count (2 bytes)
///   [24..]   First EDK entry starts here: Key Provider ID Length (2 bytes)
const FIRST_EDK_OFFSET: usize = 24;

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_key_provider_id_length_is_2_bytes() {
    //= specification/data-format/message-header.md#key-provider-id-length
    //= type=test
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    let ct = encrypt_v1(b"key provider id length test").await;

    // Verify the AAD section is empty (2 bytes of zeros)
    assert_eq!(
        u16::from_be_bytes([ct[20], ct[21]]),
        0,
        "AAD key-value pairs length must be 0 for empty EC"
    );

    // EDK count should be 1
    let edk_count = u16::from_be_bytes([ct[22], ct[23]]);
    assert_eq!(edk_count, 1, "should have exactly 1 EDK");

    // The Key Provider ID Length field is at FIRST_EDK_OFFSET and is exactly 2 bytes.
    // Read those 2 bytes as a big-endian u16.
    let key_provider_id_len =
        u16::from_be_bytes([ct[FIRST_EDK_OFFSET], ct[FIRST_EDK_OFFSET + 1]]);

    // The key provider ID is the namespace "child0 Namespace" (16 bytes).
    let expected_ns = "child0 Namespace";
    assert_eq!(
        key_provider_id_len as usize,
        expected_ns.len(),
        "Key Provider ID Length field (2 bytes) must equal the namespace length"
    );

    // Verify the actual key provider ID bytes match
    let id_start = FIRST_EDK_OFFSET + 2;
    let id_end = id_start + key_provider_id_len as usize;
    let key_provider_id = std::str::from_utf8(&ct[id_start..id_end]).unwrap();
    assert_eq!(
        key_provider_id, expected_ns,
        "Key Provider ID must match the keyring namespace"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_key_provider_id_length_serialized_as_uint16() {
    //= specification/data-format/message-header.md#key-provider-id-length
    //= type=test
    //# The key provider ID length MUST be serialized as a UInt16.
    let ct = encrypt_v1(b"uint16 serialization test").await;

    // The Key Provider ID Length is at FIRST_EDK_OFFSET, serialized as big-endian UInt16.
    let len_bytes = &ct[FIRST_EDK_OFFSET..FIRST_EDK_OFFSET + 2];
    let key_provider_id_len = u16::from_be_bytes([len_bytes[0], len_bytes[1]]);

    // "child0 Namespace" is 16 bytes = 0x0010 in big-endian UInt16
    assert_eq!(len_bytes, &[0x00, 0x10], "Key Provider ID Length must be big-endian UInt16");
    assert_eq!(
        key_provider_id_len, 16,
        "Key Provider ID Length must decode to 16 (length of 'child0 Namespace')"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_key_provider_information_length_serialized_as_uint16() {
    //= specification/data-format/message-header.md#key-provider-information-length
    //= type=test
    //# The key provider information length MUST be serialized as a UInt16.
    let ct = encrypt_v1(b"key provider info length test").await;

    // Parse Key Provider ID Length to find the Key Provider Information Length field.
    let key_provider_id_len =
        u16::from_be_bytes([ct[FIRST_EDK_OFFSET], ct[FIRST_EDK_OFFSET + 1]]) as usize;

    // Key Provider Information Length starts after Key Provider ID Length (2) + Key Provider ID (variable)
    let info_len_offset = FIRST_EDK_OFFSET + 2 + key_provider_id_len;
    let info_len_bytes = &ct[info_len_offset..info_len_offset + 2];
    let key_provider_info_len = u16::from_be_bytes([info_len_bytes[0], info_len_bytes[1]]);

    // The field is 2 bytes, big-endian UInt16, and its value must match the actual info length.
    assert!(
        key_provider_info_len > 0,
        "Key Provider Information Length must be non-zero for a raw AES keyring EDK"
    );

    // Verify the actual key provider info bytes are present at the expected offset.
    let info_start = info_len_offset + 2;
    let info_end = info_start + key_provider_info_len as usize;
    assert!(
        info_end <= ct.len(),
        "Key Provider Information must fit within the ciphertext"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_encrypted_data_key_length_serialized_as_uint16() {
    //= specification/data-format/message-header.md#encrypted-data-key-length
    //= type=test
    //# The encrypted data key length MUST be serialized as a UInt16.
    let ct = encrypt_v1(b"encrypted data key length test").await;

    // Parse through the EDK fields to find the Encrypted Data Key Length field.
    let key_provider_id_len =
        u16::from_be_bytes([ct[FIRST_EDK_OFFSET], ct[FIRST_EDK_OFFSET + 1]]) as usize;
    let info_len_offset = FIRST_EDK_OFFSET + 2 + key_provider_id_len;
    let key_provider_info_len =
        u16::from_be_bytes([ct[info_len_offset], ct[info_len_offset + 1]]) as usize;

    // Encrypted Data Key Length starts after Key Provider Info Length (2) + Key Provider Info (variable)
    let edk_len_offset = info_len_offset + 2 + key_provider_info_len;
    let edk_len_bytes = &ct[edk_len_offset..edk_len_offset + 2];
    let encrypted_data_key_len = u16::from_be_bytes([edk_len_bytes[0], edk_len_bytes[1]]);

    // The field is 2 bytes, big-endian UInt16, and its value must match the actual EDK length.
    assert!(
        encrypted_data_key_len > 0,
        "Encrypted Data Key Length must be non-zero"
    );

    // Verify the actual encrypted data key bytes are present at the expected offset.
    let edk_start = edk_len_offset + 2;
    let edk_end = edk_start + encrypted_data_key_len as usize;
    assert!(
        edk_end <= ct.len(),
        "Encrypted Data Key must fit within the ciphertext"
    );
}
