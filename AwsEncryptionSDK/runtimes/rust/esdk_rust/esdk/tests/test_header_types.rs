// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for message-header.md: supported-versions, supported-types, version, type, content-type

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

/// Encrypt with default (V2) algorithm suite.
async fn encrypt_v2(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a V1 (non-committing) algorithm suite.
async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Find the content type byte offset in a V2 ciphertext.
/// V2 header: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1).
fn content_type_offset_v2(ct: &[u8]) -> usize {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }

    // EDKs: 2-byte count, then for each: provider_id(2+len) + provider_info(2+len) + ciphertext(2+len)
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

    pos
}

#[tokio::test(flavor = "multi_thread")]
async fn test_version_v2_value() {
    //= specification/data-format/message-header.md#supported-versions
    //= type=test
    //# The supported versions MUST be:
    //= specification/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `02` MUST be version 2.0
    let ct = encrypt_v2(b"v2 version test").await;
    assert_eq!(ct[0], 0x02, "V2 ciphertext must start with version byte 0x02");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_version_v1_value() {
    //= specification/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `01` MUST be version 1.0
    let ct = encrypt_v1(b"v1 version test").await;
    assert_eq!(ct[0], 0x01, "V1 ciphertext must start with version byte 0x01");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_type_customer_aed_value() {
    //= specification/data-format/message-header.md#supported-types
    //= type=test
    //# The supported types MUST be:
    //= specification/data-format/message-header.md#supported-types
    //= type=test
    //# - `80` MUST be Customer Authenticated Encrypted Data
    let ct = encrypt_v1(b"type test").await;
    // V1 header: Version(1) + Type(1), so type byte is at offset 1
    assert_eq!(ct[1], 0x80, "V1 ciphertext must have type byte 0x80 at offset 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_type_invalid_value_rejected() {
    //= specification/data-format/message-header.md#type
    //= type=test
    //# The type (hex) of this field MUST be a value that exists in the following table:
    let keyring = test_keyring().await;
    let mut ct = encrypt_v1(b"invalid type test").await;
    // V1 header: type byte is at offset 1
    ct[1] = 0x00; // invalid type

    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert!(decrypt(&dec_input).await.is_err(), "invalid type byte 0x00 must be rejected");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_framed_value() {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# - `02` for [Framed](message-body.md#framed-data)
    let ct = encrypt_v2(b"test").await;
    let offset = content_type_offset_v2(&ct);
    assert_eq!(ct[offset], 0x02, "framed content type must be 0x02");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_nonframed_value() {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# - `01` for [Non-Framed](message-body.md#non-framed-data)
    let keyring = test_keyring().await;
    let mut ct = encrypt_v2(b"test").await;

    // Set content type byte to 0x01 (NonFramed). Decryption will fail because
    // the body format doesn't match, but the error must NOT be "Unsupported Content Type" —
    // proving 0x01 is a supported content type value.
    let offset = content_type_offset_v2(&ct);
    ct[offset] = 0x01;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        !msg.contains("Unsupported Content Type"),
        "0x01 must be accepted as a valid content type, got: {msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_invalid_value_rejected() {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# The supported content types MUST be:
    let keyring = test_keyring().await;
    let mut ct = encrypt_v2(b"test").await;

    let offset = content_type_offset_v2(&ct);
    ct[offset] = 0x00; // invalid content type

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(decrypt(&dec_input).await.is_err(), "invalid content type 0x00 must be rejected");
}
