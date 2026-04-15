// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md#encrypted-data-keys

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;


#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_keys_serialization_order() {
    // Create two keyrings so we get 2 EDKs in the message
    //= specification/data-format/message-header.md#encrypted-data-keys
    //= type=test
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.
    let keyring1 = test_keyring().await;
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
        let ct = encrypt_with_version(b"edk serialization order", version, multi_keyring.clone()).await;
        let count_offset = edk_count_offset(&ct, version);
        let edk_count = u16::from_be_bytes([ct[count_offset], ct[count_offset + 1]]);
        assert_eq!(edk_count, 2, "{version:?}: multi-keyring must produce EDK count of 2");

        let first_edk_offset = count_offset + 2;
        let (edk1_id, _, _, after_edk1) = parse_edk_at(&ct, first_edk_offset);
        let (edk2_id, _, _, _) = parse_edk_at(&ct, after_edk1);

        assert_eq!(edk1_id, "child0 Namespace", "{version:?}: EDK 1 provider ID");
        assert_eq!(edk2_id, "child1 Namespace", "{version:?}: EDK 2 provider ID");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_is_2_bytes_uint16() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"test edk count", version, keyring.clone()).await;
        let offset = edk_count_offset(&ct, version);
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The length of the serialized encrypted data key count MUST be 2 bytes.
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The encrypted data key count MUST be interpreted as a UInt16.
        let edk_count = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        assert_eq!(edk_count, 1, "{version:?}: single keyring must produce exactly 1 EDK");

        // Round-trip proves the count was correctly serialized
        let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        let result = decrypt(&dec_input).await.unwrap();
        assert_eq!(result.plaintext, b"test edk count");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_must_be_greater_than_zero() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"test zero edks", version, keyring.clone()).await;
        let offset = edk_count_offset(&ct, version);
        // Tampering test; set to 0 on message to create failure condition
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# This value MUST be greater than 0.
        ct[offset] = 0;
        ct[offset + 1] = 0;

        let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: message with 0 encrypted data keys must be rejected"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_must_not_exceed_max() {
    let keyring1 = test_keyring().await;
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
        let mut enc_input = EncryptInput::with_legacy_keyring(
            b"max edk count test",
            EncryptionContext::new(),
            multi_keyring.clone(),
        );
        if let Version::V1 = version {
            enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
            enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }

        // 2 EDKs, max=1 → must fail
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# This value MUST be less than or equal to the
        //# [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys)
        //# if the maximum number is configured.
        enc_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
        assert!(
            encrypt(&enc_input).await.is_err(),
            "{version:?}: encrypt must fail when EDK count exceeds max"
        );

        // 2 EDKs, max=2 → must succeed
        enc_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(2).unwrap());
        assert!(
            encrypt(&enc_input).await.is_ok(),
            "{version:?}: encrypt must succeed when EDK count equals max"
        );

        // 2 EDKs, no max → must succeed
        enc_input.max_encrypted_data_keys = None;
        assert!(
            encrypt(&enc_input).await.is_ok(),
            "{version:?}: encrypt must succeed when max is not configured"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_serialization_order() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk entry order test", version, keyring.clone()).await;
        let count_offset = edk_count_offset(&ct, version);
        let first_edk_offset = count_offset + 2;

        //= specification/data-format/message-header.md#encrypted-data-key-entries
        //= type=test
        //# Each Encrypted Data Key Entry MUST consist of, in order,
        //# Key Provider ID Length,
        //# Key Provider ID,
        //# Key Provider Information Length,
        //# Key Provider Information,
        //# Encrypted Data Key Length,
        //# and Encrypted Data Key.
        let (kp_id_len, kp_id, kp_info_len, kp_info, edk_len, edk_data) =
            parse_edk_raw_at(&ct, first_edk_offset);

        assert_eq!(kp_id_len as usize, kp_id.len(), "{version:?}");
        assert_eq!(std::str::from_utf8(kp_id).unwrap(), "child0 Namespace", "{version:?}");
        assert_eq!(kp_info_len as usize, kp_info.len(), "{version:?}");
        assert_eq!(edk_len as usize, edk_data.len(), "{version:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_matches_data() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"kp id length test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The length of the serialized key provider ID length field MUST be 2 bytes.
        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The key provider ID length MUST be interpreted as a UInt16.
        let (kp_id_len, kp_id, _, _, _, _) = parse_edk_raw_at(&ct, first_edk_offset);
        //= specification/data-format/message-header.md#key-provider-id
        //= type=test
        //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
        assert_eq!(kp_id_len as usize, kp_id.len(), "{version:?}: Key Provider ID length must match data");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_is_utf8() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"kp id utf8 test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (_, kp_id, _, _, _, _) = parse_edk_raw_at(&ct, first_edk_offset);
        //= specification/data-format/message-header.md#key-provider-id
        //= type=test
        //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
        let kp_id_str = std::str::from_utf8(kp_id).expect("Key Provider ID must be valid UTF-8");
        assert_eq!(kp_id_str, "child0 Namespace", "{version:?}: Key Provider ID must match keyring namespace");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_length_field_is_2_bytes_uint16() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"kp info length test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (kp_id_len, _, kp_info_len, kp_info, _, _) = parse_edk_raw_at(&ct, first_edk_offset);

        // The info length field is at first_edk_offset + 2 + kp_id_len, and is 2 bytes
        let info_len_offset = first_edk_offset + 2 + kp_id_len as usize;
        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The length of the serialized key provider information length field MUST be 2 bytes.
        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The key provider information length MUST be interpreted as a UInt16.
        let read_len = u16::from_be_bytes([ct[info_len_offset], ct[info_len_offset + 1]]);
        assert_eq!(read_len, kp_info_len, "{version:?}: info length field must be 2-byte UInt16");
        assert_eq!(kp_info_len as usize, kp_info.len(), "{version:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_length_matches_data() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"kp info data test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (_, _, kp_info_len, kp_info, _, _) = parse_edk_raw_at(&ct, first_edk_offset);
        //= specification/data-format/message-header.md#key-provider-information
        //= type=test
        //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
        assert_eq!(kp_info_len as usize, kp_info.len(), "{version:?}: info length must match data");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_information_is_bytes() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"kp info bytes test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (_, _, kp_info_len, kp_info, _, _) = parse_edk_raw_at(&ct, first_edk_offset);
        // Raw AES keyring provider info format:
        //   key_name || tag_len_bits(u32be) || iv_len_bytes(u32be) || iv
        //= specification/data-format/message-header.md#key-provider-information
        //= type=test
        //# The key provider information MUST be interpreted as bytes.
        let key_name = b"child0 Name";
        assert!(kp_info_len as usize >= key_name.len() + 4 + 4 + 12,
            "{version:?}: provider info too short for raw AES keyring format");
        assert_eq!(&kp_info[..key_name.len()], &key_name[..],
            "{version:?}: provider info must start with key name");
        let tag_len_bits = u32::from_be_bytes(kp_info[key_name.len()..key_name.len() + 4].try_into().unwrap());
        assert_eq!(tag_len_bits, 128, "{version:?}: tag length must be 128 bits");
        let iv_len_bytes = u32::from_be_bytes(kp_info[key_name.len() + 4..key_name.len() + 8].try_into().unwrap());
        assert_eq!(iv_len_bytes, 12, "{version:?}: IV length must be 12 bytes");
        let iv = &kp_info[key_name.len() + 8..];
        assert_eq!(iv.len(), 12, "{version:?}: IV must be exactly 12 bytes");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_field_is_2_bytes_uint16() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk length field test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (kp_id_len, _, kp_info_len, _, edk_len, edk_data) = parse_edk_raw_at(&ct, first_edk_offset);

        // EDK length field is at: first_edk_offset + 2 + kp_id_len + 2 + kp_info_len
        let edk_len_offset = first_edk_offset + 2 + kp_id_len as usize + 2 + kp_info_len as usize;
        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The length of the serialized encrypted data key length field MUST be 2 bytes.
        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The encrypted data key length MUST be interpreted as a UInt16.
        let read_len = u16::from_be_bytes([ct[edk_len_offset], ct[edk_len_offset + 1]]);
        assert_eq!(read_len, edk_len, "{version:?}: EDK length field must be 2-byte UInt16");
        assert_eq!(edk_len as usize, edk_data.len(), "{version:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_length_matches_data() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk data length test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        let (_, _, _, _, edk_len, edk_data) = parse_edk_raw_at(&ct, first_edk_offset);
        //= specification/data-format/message-header.md#encrypted-data-key
        //= type=test
        //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
        assert_eq!(edk_len as usize, edk_data.len(), "{version:?}: EDK length must match data");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_is_bytes() {
    let keyring = test_keyring().await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk bytes test", version, keyring.clone()).await;
        let first_edk_offset = edk_count_offset(&ct, version) + 2;
        //= specification/data-format/message-header.md#encrypted-data-key
        //= type=test
        //# The encrypted data key MUST be interpreted as bytes.
        let (_, _, _, _, edk_len, edk_data) = parse_edk_raw_at(&ct, first_edk_offset);
        assert!(edk_len > 0, "{version:?}: Encrypted Data Key should be non-empty");
        // For raw AES keyring with AES-256-GCM: ciphertext = encrypted_key + GCM_tag(16 bytes)
        // The data key length depends on the algorithm suite, but the GCM tag is always 16 bytes.
        // Verify we can read the full ciphertext and it ends with a 16-byte tag region.
        let copied: Vec<u8> = edk_data.to_vec();
        assert_eq!(copied.len(), edk_len as usize, "{version:?}: all EDK bytes must be readable");
        assert!(copied.len() > 16, "{version:?}: EDK ciphertext must be longer than just a GCM tag");
        // The ciphertext should not be all zeros (it's encrypted key material)
        assert!(copied.iter().any(|&b| b != 0), "{version:?}: EDK ciphertext must contain non-zero bytes");
    }
}
