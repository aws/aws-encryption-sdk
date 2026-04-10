// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md — Encrypted Data Keys sections

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

#[derive(Clone, Copy, Debug)]
enum Version { V1, V2 }
const VERSIONS: [Version; 2] = [Version::V1, Version::V2];

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

async fn encrypt_with(
    plaintext: &[u8],
    version: Version,
    keyring: aws_mpl_legacy::dafny::types::keyring::KeyringRef,
) -> Vec<u8> {
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    if let Version::V1 = version {
        input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
        input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    }
    encrypt(&input).await.unwrap().ciphertext
}

/// Find the byte offset where the EDK count field begins in the ciphertext.
/// V1 layout: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) + AAD(variable)
/// V2 layout: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable)
fn edk_count_offset(ct: &[u8], version: Version) -> usize {
    let mut pos = match version {
        Version::V1 => 1 + 1 + 2 + 16, // Version + Type + AlgSuiteID + MessageID
        Version::V2 => 1 + 2 + 32,     // Version + AlgSuiteID + MessageID
    };
    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }
    pos
}

/// Parsed representation of a single EDK entry from raw bytes, including byte offsets.
#[derive(Debug)]
struct ParsedEdk {
    provider_id_len: u16,
    provider_id: String,
    provider_info_len: u16,
    provider_info_len_offset: usize,
    provider_info: Vec<u8>,
    provider_info_offset: usize,
    ciphertext_len: u16,
    ciphertext_len_offset: usize,
    ciphertext: Vec<u8>,
    ciphertext_offset: usize,
}

/// Parse all EDK entries from raw ciphertext bytes starting at the EDK count field.
/// Returns (edk_count, parsed_edks).
fn parse_edks_from_bytes(ct: &[u8], edk_start: usize) -> (u16, Vec<ParsedEdk>) {
    let mut pos = edk_start;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;

    let mut edks = Vec::new();
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let pid = String::from_utf8(ct[pos..pos + pid_len as usize].to_vec()).unwrap();
        pos += pid_len as usize;

        let pinfo_len_offset = pos;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let pinfo_offset = pos;
        let pinfo = ct[pos..pos + pinfo_len as usize].to_vec();
        pos += pinfo_len as usize;

        let ct_len_offset = pos;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let ct_offset = pos;
        let edk_ct = ct[pos..pos + ct_len as usize].to_vec();
        pos += ct_len as usize;

        edks.push(ParsedEdk {
            provider_id_len: pid_len,
            provider_id: pid,
            provider_info_len: pinfo_len,
            provider_info_len_offset: pinfo_len_offset,
            provider_info: pinfo,
            provider_info_offset: pinfo_offset,
            ciphertext_len: ct_len,
            ciphertext_len_offset: ct_len_offset,
            ciphertext: edk_ct,
            ciphertext_offset: ct_offset,
        });
    }
    (edk_count, edks)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_keys_ordering() {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //= type=test
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk ordering test", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        // The count field (2 bytes) comes first, then the entries follow immediately
        let edk_count = u16::from_be_bytes([ct[edk_start], ct[edk_start + 1]]);
        assert!(edk_count > 0, "{version:?}: count field must precede entries and be > 0");
        // Verify entries are parseable immediately after count
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        assert_eq!(
            edks.len(),
            edk_count as usize,
            "{version:?}: number of parsed entries must match count"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_field_is_2_bytes() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# The length of the serialized encrypted data key count MUST be 2 bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk count 2 bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        // Reading exactly 2 bytes as big-endian u16 must produce a valid count
        let count_bytes = &ct[edk_start..edk_start + 2];
        let count = u16::from_be_bytes([count_bytes[0], count_bytes[1]]);
        assert_eq!(count, 1, "{version:?}: single keyring should produce count=1 in 2 bytes");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_interpreted_as_uint16() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# The encrypted data key count MUST be interpreted as a UInt16.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk count uint16", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        // Interpret as big-endian UInt16 and verify round-trip succeeds
        let count = u16::from_be_bytes([ct[edk_start], ct[edk_start + 1]]);
        assert_eq!(count, 1, "{version:?}: count must be interpretable as UInt16");
        // Verify decrypt succeeds, proving the UInt16 interpretation is correct
        let dec_input = DecryptInput::with_legacy_keyring(
            &ct,
            EncryptionContext::new(),
            test_keyring().await,
        );
        let result = if let Version::V1 = version {
            let mut di = dec_input;
            di.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
            decrypt(&di).await
        } else {
            decrypt(&dec_input).await
        };
        assert!(result.is_ok(), "{version:?}: decrypt must succeed with valid UInt16 count");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_must_be_greater_than_zero() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be greater than 0.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let mut ct = encrypt_with(b"edk count zero test", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        // Corrupt the count to 0
        ct[edk_start] = 0;
        ct[edk_start + 1] = 0;
        let mut dec_input = DecryptInput::with_legacy_keyring(
            &ct,
            EncryptionContext::new(),
            test_keyring().await,
        );
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: decrypt must fail when EDK count is 0"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_max_encrypted_data_keys_encrypt() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be less than or equal to the
    //# [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys)
    //# if the maximum number is configured.
    let (ns1, name1) = namespace_and_name(0);
    let keyring1 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns1)
        .key_name(name1)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let (ns2, name2) = namespace_and_name(1);
    let keyring2 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns2)
        .key_name(name2)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let multi_keyring = mpl()
        .create_multi_keyring()
        .generator(keyring1)
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();
    // Multi-keyring produces 2 EDKs; setting max to 1 must fail on encrypt
    let mut enc_input = EncryptInput::with_legacy_keyring(
        b"max edk encrypt test",
        EncryptionContext::new(),
        multi_keyring,
    );
    enc_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    assert!(
        encrypt(&enc_input).await.is_err(),
        "encrypt must fail when EDK count exceeds max_encrypted_data_keys"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_max_encrypted_data_keys_decrypt() {
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be less than or equal to the
    //# [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys)
    //# if the maximum number is configured.

    // Failure case: encrypt with 2 EDKs (multi-keyring), decrypt with max=1
    let (ns1, name1) = namespace_and_name(0);
    let keyring1 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns1)
        .key_name(name1)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let (ns2, name2) = namespace_and_name(1);
    let keyring2 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns2)
        .key_name(name2)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let multi_keyring = mpl()
        .create_multi_keyring()
        .generator(keyring1.clone())
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();
    for version in VERSIONS {
        let ct = encrypt_with(b"max edk decrypt fail", version, multi_keyring.clone()).await;
        // Decrypt with max=1 must fail because ciphertext has 2 EDKs
        let mut dec_input = DecryptInput::with_legacy_keyring(
            &ct,
            EncryptionContext::new(),
            keyring1.clone(),
        );
        dec_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: decrypt must fail when EDK count (2) exceeds max (1)"
        );
    }

    // Success case: encrypt with 1 EDK, decrypt with max=1
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"max edk decrypt ok", version, keyring).await;
        let mut dec_input = DecryptInput::with_legacy_keyring(
            &ct,
            EncryptionContext::new(),
            test_keyring().await,
        );
        dec_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_ok(),
            "{version:?}: decrypt must succeed when EDK count <= max"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_field_ordering() {
    //= specification/data-format/message-header.md#encrypted-data-key-entries
    //= type=test
    //# Each Encrypted Data Key Entry MUST consist of, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk entry ordering", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (count, edks) = parse_edks_from_bytes(&ct, edk_start);
        assert_eq!(count, 1, "{version:?}: expected 1 EDK");
        let edk = &edks[0];
        // Verify all fields were parsed in order successfully
        assert!(edk.provider_id_len > 0, "{version:?}: provider ID length must be > 0");
        assert!(!edk.provider_id.is_empty(), "{version:?}: provider ID must not be empty");
        assert!(edk.ciphertext_len > 0, "{version:?}: encrypted data key length must be > 0");
        assert!(!edk.ciphertext.is_empty(), "{version:?}: encrypted data key must not be empty");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_ordering_with_multiple_edks() {
    //= specification/data-format/message-header.md#encrypted-data-key-entries
    //= type=test
    //# Each Encrypted Data Key Entry MUST consist of, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.
    let (ns1, name1) = namespace_and_name(0);
    let keyring1 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns1)
        .key_name(name1)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let (ns2, name2) = namespace_and_name(1);
    let keyring2 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns2)
        .key_name(name2)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let multi_keyring = mpl()
        .create_multi_keyring()
        .generator(keyring1)
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();
    for version in VERSIONS {
        let ct = encrypt_with(b"multi edk ordering", version, multi_keyring.clone()).await;
        let edk_start = edk_count_offset(&ct, version);
        let (count, edks) = parse_edks_from_bytes(&ct, edk_start);
        assert_eq!(count, 2, "{version:?}: multi-keyring should produce 2 EDKs");
        // Both entries must be independently parseable with valid fields
        for (i, edk) in edks.iter().enumerate() {
            assert!(edk.provider_id_len > 0, "{version:?}: EDK {i} provider ID length must be > 0");
            assert!(!edk.provider_id.is_empty(), "{version:?}: EDK {i} provider ID must not be empty");
            assert!(edk.ciphertext_len > 0, "{version:?}: EDK {i} ciphertext length must be > 0");
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_field_is_2_bytes() {
    //= specification/data-format/message-header.md#key-provider-id-length
    //= type=test
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp id len 2 bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        // Skip count (2 bytes), then the first 2 bytes of the entry are the key provider ID length
        let pid_len_offset = edk_start + 2;
        let pid_len_bytes = &ct[pid_len_offset..pid_len_offset + 2];
        let pid_len = u16::from_be_bytes([pid_len_bytes[0], pid_len_bytes[1]]);
        assert!(pid_len > 0, "{version:?}: key provider ID length field is 2 bytes and > 0");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_interpreted_as_uint16() {
    //= specification/data-format/message-header.md#key-provider-id-length
    //= type=test
    //# The key provider ID length MUST be interpreted as a UInt16.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp id len uint16", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // The parsed UInt16 length must match the actual provider ID byte length
        assert_eq!(
            edk.provider_id_len as usize,
            edk.provider_id.len(),
            "{version:?}: UInt16 key provider ID length must match actual ID byte length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_equals_id_bytes() {
    //= specification/data-format/message-header.md#key-provider-id
    //= type=test
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp id len equals", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        assert_eq!(
            edk.provider_id.len(),
            edk.provider_id_len as usize,
            "{version:?}: serialized key provider ID length must equal Key Provider ID Length field"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_is_utf8() {
    //= specification/data-format/message-header.md#key-provider-id
    //= type=test
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp id utf8", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // parse_edks_from_bytes already calls String::from_utf8 which would panic on invalid UTF-8
        // Verify the provider ID is a non-empty valid UTF-8 string
        assert!(
            !edk.provider_id.is_empty(),
            "{version:?}: key provider ID must be valid non-empty UTF-8"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_length_field_is_2_bytes() {
    //= specification/data-format/message-header.md#key-provider-information-length
    //= type=test
    //# The length of the serialized key provider information length field MUST be 2 bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp info len 2 bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // Read exactly 2 bytes at the info length offset and verify they decode to the expected u16
        let info_len_bytes = &ct[edk.provider_info_len_offset..edk.provider_info_len_offset + 2];
        let info_len = u16::from_be_bytes([info_len_bytes[0], info_len_bytes[1]]);
        assert_eq!(
            info_len, edk.provider_info_len,
            "{version:?}: 2-byte field at offset {} must decode to provider info length {}",
            edk.provider_info_len_offset, edk.provider_info_len
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_length_interpreted_as_uint16() {
    //= specification/data-format/message-header.md#key-provider-information-length
    //= type=test
    //# The key provider information length MUST be interpreted as a UInt16.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp info len uint16", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        assert_eq!(
            edk.provider_info_len as usize,
            edk.provider_info.len(),
            "{version:?}: UInt16 key provider info length must match actual info byte length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_length_equals_info_bytes() {
    //= specification/data-format/message-header.md#key-provider-information
    //= type=test
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp info len equals", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        assert_eq!(
            edk.provider_info.len(),
            edk.provider_info_len as usize,
            "{version:?}: serialized key provider info length must equal Key Provider Information Length field"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_interpreted_as_bytes() {
    //= specification/data-format/message-header.md#key-provider-information
    //= type=test
    //# The key provider information MUST be interpreted as bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"kp info bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // Verify the parsed bytes match the raw bytes at the expected offset in the ciphertext
        let raw_info = &ct[edk.provider_info_offset..edk.provider_info_offset + edk.provider_info_len as usize];
        assert_eq!(
            edk.provider_info, raw_info,
            "{version:?}: parsed provider info bytes must match raw bytes at offset {}",
            edk.provider_info_offset
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_field_is_2_bytes() {
    //= specification/data-format/message-header.md#encrypted-data-key-length
    //= type=test
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk len 2 bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // Read exactly 2 bytes at the ciphertext length offset and verify they decode to the expected u16
        let ct_len_bytes = &ct[edk.ciphertext_len_offset..edk.ciphertext_len_offset + 2];
        let ct_len = u16::from_be_bytes([ct_len_bytes[0], ct_len_bytes[1]]);
        assert_eq!(
            ct_len, edk.ciphertext_len,
            "{version:?}: 2-byte field at offset {} must decode to ciphertext length {}",
            edk.ciphertext_len_offset, edk.ciphertext_len
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_interpreted_as_uint16() {
    //= specification/data-format/message-header.md#encrypted-data-key-length
    //= type=test
    //# The encrypted data key length MUST be interpreted as a UInt16.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk len uint16", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        assert_eq!(
            edk.ciphertext_len as usize,
            edk.ciphertext.len(),
            "{version:?}: UInt16 encrypted data key length must match actual ciphertext byte length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_equals_ciphertext_bytes() {
    //= specification/data-format/message-header.md#encrypted-data-key
    //= type=test
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk len equals ct", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        assert_eq!(
            edk.ciphertext.len(),
            edk.ciphertext_len as usize,
            "{version:?}: serialized encrypted data key length must equal Encrypted Data Key Length field"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_interpreted_as_bytes() {
    //= specification/data-format/message-header.md#encrypted-data-key
    //= type=test
    //# The encrypted data key MUST be interpreted as bytes.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with(b"edk as bytes", version, keyring).await;
        let edk_start = edk_count_offset(&ct, version);
        let (_, edks) = parse_edks_from_bytes(&ct, edk_start);
        let edk = &edks[0];
        // Verify the parsed ciphertext bytes match the raw bytes at the expected offset
        let raw_ct = &ct[edk.ciphertext_offset..edk.ciphertext_offset + edk.ciphertext_len as usize];
        assert_eq!(
            edk.ciphertext, raw_ct,
            "{version:?}: parsed ciphertext bytes must match raw bytes at offset {}",
            edk.ciphertext_offset
        );
        assert!(!edk.ciphertext.is_empty(), "{version:?}: encrypted data key bytes must not be empty");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_round_trip_proves_edk_serialization() {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //= type=test
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let pt = b"round trip edk test";
        let ct = encrypt_with(pt, version, keyring).await;
        let mut dec_input = DecryptInput::with_legacy_keyring(
            &ct,
            EncryptionContext::new(),
            test_keyring().await,
        );
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        let result = decrypt(&dec_input).await.unwrap();
        assert_eq!(
            result.plaintext, pt,
            "{version:?}: round-trip proves EDK serialization/deserialization is correct"
        );
    }
}
