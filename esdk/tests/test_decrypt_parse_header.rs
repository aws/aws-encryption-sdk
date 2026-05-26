// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#parse-the-header
//! and spec/client-apis/decrypt.md#behavior

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_version_rejected() {
    //= spec/client-apis/decrypt.md#parse-the-header
    //= type=test
    //# The value MUST be a [supported version](../data-format/message-header.md#supported-versions).
    let keyring = test_keyring().await;
    let plaintext = b"unsupported version test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Byte 0 is the version field — set to an unsupported value
    ct[0] = 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when version byte is unsupported");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_content_type_v1_rejected() {
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    let keyring = test_keyring().await;
    let plaintext = b"unsupported content type v1 test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 header layout with empty encryption context:
    // version(1) + type(1) + alg_suite(2) + message_id(16) = 20 bytes fixed
    // AAD section with empty EC: key_value_pairs_length(2) = 0x0000 → 2 bytes total
    // EDK section: edk_count(2) + each EDK
    let mut pos: usize = 20; // after version + type + alg_suite + message_id
    // Skip AAD section: read key_value_pairs_length
    let aad_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2; // past key_value_pairs_length
    if aad_len > 0 {
        // aad_len already includes the 2-byte key_value_pair_count
        pos += aad_len;
    }
    // Skip EDK section: edk_count(2) + each EDK
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let provider_id_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + provider_id_len;
        let provider_info_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + provider_info_len;
        let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + edk_len;
    }
    // pos now points to the content type byte
    let original_content_type = ct[pos];
    assert!(
        original_content_type == 1 || original_content_type == 2,
        "sanity check: content type should be 1 or 2, got {original_content_type}"
    );
    ct[pos] = 0xFF; // unsupported content type

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when content type byte is unsupported");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_content_type_v2_rejected() {
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    let keyring = test_keyring().await;
    let plaintext = b"unsupported content type v2 test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V2 header layout with empty encryption context:
    // version(1) + alg_suite(2) + message_id(32) = 35 bytes fixed
    // AAD section with empty EC: key_value_pairs_length(2) = 0x0000 → 2 bytes total
    // EDK section: edk_count(2) + each EDK
    let mut pos: usize = 35; // after version + alg_suite + message_id (32 bytes for V2)
    // Skip AAD section: read key_value_pairs_length
    let aad_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2; // past key_value_pairs_length
    if aad_len > 0 {
        // aad_len already includes the 2-byte key_value_pair_count
        pos += aad_len;
    }
    // Skip EDK section: edk_count(2) + each EDK
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let provider_id_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + provider_id_len;
        let provider_info_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + provider_info_len;
        let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + edk_len;
    }
    // pos now points to the content type byte
    let original_content_type = ct[pos];
    assert!(
        original_content_type == 1 || original_content_type == 2,
        "sanity check: content type should be 1 or 2, got {original_content_type}"
    );
    ct[pos] = 0xFF; // unsupported content type

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when V2 content type byte is unsupported");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_type_rejected() {
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //# The value MUST be a [supported type](../data-format/message-header.md#supported-types).
    let keyring = test_keyring().await;
    let plaintext = b"unsupported type test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 layout: byte 0 = version (0x01), byte 1 = type (0x80)
    assert_eq!(ct[1], 0x80, "sanity check: V1 type should be 0x80");
    ct[1] = 0xFF; // unsupported type

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when message type byte is unsupported");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_trailing_bytes_after_message_rejected() {
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //# - If this operation successfully completes the above steps
    //# but there are consumable bytes which are intended to be decrypted,
    //# this operation MUST fail.
    let keyring = test_keyring().await;
    let plaintext = b"trailing bytes test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Append extra bytes after the valid message
    ct.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when there are trailing bytes after the message");
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_max_encrypted_data_keys_enforcement() {
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    // Create two keyrings and a multi-keyring to produce 2 EDKs
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
        .generator(keyring1.clone())
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();

    let plaintext = b"max edk decrypt test";
    let enc_input = EncryptInput::with_legacy_keyring(
        plaintext,
        EncryptionContext::new(),
        multi_keyring.clone(),
    );
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with max_encrypted_data_keys=1, but message has 2 EDKs → must fail
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), multi_keyring);
    dec_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when EDK count exceeds max_encrypted_data_keys");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_header_info_released_before_verification() {
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //# Until the [header is verified](#verify-the-header), this operation MUST NOT
    //# release any parsed information from the header.
    // Tamper with the header auth tag so header verification fails.
    // The non-streaming decrypt must return an error with no partial output.
    let keyring = test_keyring().await;
    let plaintext = b"no header info before verification";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with a byte in the header body to cause header auth tag verification failure
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[10] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail entirely when header verification fails — no partial header info released");
    assert_eq!(err.kind, ErrorKind::ValidationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_tag_deserialized_and_verified() {
    // Tamper the V1 header auth tag to prove it was deserialized and used for verification.
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth tag tamper test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Baseline: untampered V1 message decrypts.
    let mut baseline_input = DecryptInput::from_encrypt(&ct, &enc_input);
    baseline_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert!(decrypt(&baseline_input).await.is_ok(), "baseline must pass");

    // V1 header auth: IV(12) + Tag(16) at the end of the header, before the body.
    // Tamper the last byte of the header (part of the auth tag).
    // V1 header ends at: fixed(20) + AAD(variable) + EDKs(variable) + content_type(1)
    //   + reserved(4) + iv_length(1) + frame_length(4) + header_auth_iv(12) + header_auth_tag(16)
    // The auth tag's last byte is just before the body starts.
    let body_start = find_body_start(&ct, 4096).expect("body start");
    let auth_tag_last_byte = body_start - 1;
    ct[auth_tag_last_byte] ^= 0xFF;

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Tampered V1 auth tag causes failure, proving it was deserialized and verified
    //# - MUST deserialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    let err = decrypt(&dec_input).await.expect_err("tampered V1 auth tag must fail");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_deserialized_and_used() {
    // Tamper the V1 header auth IV to prove it was deserialized and used for verification.
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth iv tamper test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Baseline
    let mut baseline_input = DecryptInput::from_encrypt(&ct, &enc_input);
    baseline_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert!(decrypt(&baseline_input).await.is_ok(), "baseline must pass");

    // V1 header auth IV is 12 bytes immediately before the 16-byte auth tag, which
    // is immediately before the body. Tamper its first byte.
    let body_start = find_body_start(&ct, 4096).expect("body start");
    let iv_start = body_start - 16 - 12; // 16 bytes tag + 12 bytes IV before body
    ct[iv_start] ^= 0xFF;

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Tampered V1 header auth IV causes failure, proving IV was deserialized and used
    //# - MUST deserialize the [IV](../data-format/message-header.md#iv).
    let err = decrypt(&dec_input).await.expect_err("tampered V1 header auth IV must fail");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}
