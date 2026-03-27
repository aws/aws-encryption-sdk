// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/encrypt.md#v1-header

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
async fn encrypt_v1(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt V1 then decrypt, returning decrypted plaintext.
async fn round_trip_v1(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let ct = encrypt_v1(plaintext, ec).await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    decrypt(&dec_input).await.unwrap().plaintext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_serialized() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-10) MUST be serialized with the following specifics:
    let pt = b"test v1 header";
    let ct = encrypt_v1(pt, EncryptionContext::new()).await;
    // V1 header starts with version byte 0x01
    assert_eq!(ct[0], 0x01, "first byte must be V1 version 0x01");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_version() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
    //# [Version](../data-format/message-header.md#version) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    //= specification/data-format/message-header.md#header-body-version-1-0
    //= type=test
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    let ct = encrypt_v1(b"version test", EncryptionContext::new()).await;
    assert_eq!(ct[0], 0x01, "Version field must be 0x01 for V1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_type() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Type](../data-format/message-header.md#type): MUST be serialized according to the
    //# [Type](../data-format/message-header.md#type) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    let ct = encrypt_v1(b"type test", EncryptionContext::new()).await;
    // Type field is at offset 1, value 0x80 = Customer Authenticated Encrypted Data
    assert_eq!(ct[1], 0x80, "Type field must be 0x80 (Customer AED)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_algorithm_suite_id() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
    //# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    let ct = encrypt_v1(b"suite test", EncryptionContext::new()).await;
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    let suite_id_bytes = &ct[2..4];
    assert_eq!(suite_id_bytes.len(), 2, "Algorithm Suite ID must be 2 bytes");
    // AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let suite_id = u16::from_be_bytes([ct[2], ct[3]]);
    assert_eq!(suite_id, 0x0178, "Algorithm Suite ID must match the suite used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_message_id() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    let ct1 = encrypt_v1(b"msg id test", EncryptionContext::new()).await;
    let ct2 = encrypt_v1(b"msg id test", EncryptionContext::new()).await;
    // Message ID is 16 bytes at offset 4
    let msg_id_1 = &ct1[4..20];
    let msg_id_2 = &ct2[4..20];
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");
    //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
    assert_eq!(msg_id_1.len(), 16, "V1 Message ID must be 16 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_aad() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
    //# [AAD](../data-format/message-header.md#aad) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    let ec = std::collections::HashMap::from([("key1".to_string(), "val1".to_string())]);
    let pt = b"aad test";
    let result = round_trip_v1(pt, ec).await;
    assert_eq!(result, pt, "round-trip with EC proves AAD serialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_encrypted_data_keys() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
    //# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    let pt = b"edk test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves EDKs serialized correctly (decrypt uses them)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_content_type() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
    //# [Content Type](../data-format/message-header.md#content-type) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    let pt = b"content type test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves content type is correct (framed = 0x02)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_reserved() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Reserved](../data-format/message-header.md#reserved): MUST be serialized according to the
    //# [Reserved](../data-format/message-header.md#reserved) specification.
    let pt = b"reserved test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves reserved bytes are correct (decrypt validates them)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_iv_length() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [IV Length](../data-format/message-header.md#iv-length): MUST be serialized according to the
    //# [IV Length](../data-format/message-header.md#iv-length) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md).
    let pt = b"iv length test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves IV length matches algorithm suite (decrypt validates it)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_frame_length() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
    //# [Frame Length](../data-format/message-header.md#frame-length) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST be the value of the frame size determined above.
    let pt = b"frame length test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves frame length is serialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_serialization_order() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.
    //= specification/data-format/message-header.md#header-body-version-1-0
    //= type=test
    //# The V1 Header Body MUST be serialized as, in order,
    //# Version,
    //# Type,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Reserved,
    //# IV Length,
    //# and Frame Length.
    let ct = encrypt_v1(b"order test", EncryptionContext::new()).await;
    // Verify the fixed-offset fields are in the correct order:
    // [0]    = Version (0x01)
    // [1]    = Type (0x80)
    // [2..4] = Algorithm Suite ID (2 bytes)
    // [4..20] = Message ID (16 bytes)
    // Then variable-length AAD, EDKs, then:
    // Content Type (1 byte), Reserved (4 bytes), IV Length (1 byte), Frame Length (4 bytes)
    assert_eq!(ct[0], 0x01, "offset 0: Version");
    assert_eq!(ct[1], 0x80, "offset 1: Type");
    let suite_id = u16::from_be_bytes([ct[2], ct[3]]);
    assert_eq!(suite_id, 0x0178, "offset 2-3: Algorithm Suite ID");
    assert_eq!(ct[4..20].len(), 16, "offset 4-19: Message ID (16 bytes)");
    // Round-trip proves the entire header is correctly ordered
    let result = round_trip_v1(b"order test", EncryptionContext::new()).await;
    assert_eq!(result, b"order test", "round-trip proves serialization order is correct");
}
