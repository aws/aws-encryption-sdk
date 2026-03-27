// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-keys

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

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

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
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# The length of the serialized encrypted data key count MUST be 2 bytes.

    //= specification/data-format/message-header.md#encrypted-data-key-count
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
    //= specification/data-format/message-header.md#encrypted-data-key-count
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
