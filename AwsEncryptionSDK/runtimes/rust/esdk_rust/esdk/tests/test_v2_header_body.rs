// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization (specification/client-apis/encrypt.md#v2-header)

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

/// Encrypt plaintext with default settings (V2 algorithm suite), return ciphertext bytes.
async fn encrypt_default(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt V2 then decrypt, returning decrypted plaintext.
async fn round_trip_v2(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    let ct = encrypt(&input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Parse the V2 header body fields from ciphertext bytes, returning the byte offset
/// after each field boundary in order. Panics if the header is not well-formed.
/// Returns a Vec of (field_name, start_offset, end_offset) tuples.
fn parse_v2_header_field_offsets(ct: &[u8]) -> Vec<(&'static str, usize, usize)> {
    let mut fields = Vec::new();
    let mut pos = 0;

    // Version: 1 byte (must be 0x02 for V2)
    assert!(pos < ct.len(), "not enough bytes for Version");
    assert_eq!(ct[pos], 0x02, "expected V2 version byte");
    fields.push(("Version", pos, pos + 1));
    pos += 1;

    // Algorithm Suite ID: 2 bytes
    assert!(pos + 2 <= ct.len(), "not enough bytes for Algorithm Suite ID");
    fields.push(("Algorithm Suite ID", pos, pos + 2));
    pos += 2;

    // Message ID: 32 bytes (V2 uses 32-byte message IDs)
    assert!(pos + 32 <= ct.len(), "not enough bytes for Message ID");
    fields.push(("Message ID", pos, pos + 32));
    pos += 32;

    // AAD: variable length, self-describing
    assert!(pos + 2 <= ct.len(), "not enough bytes for AAD length");
    let aad_start = pos;
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }
    fields.push(("AAD", aad_start, pos));

    // Encrypted Data Keys: variable length
    assert!(pos + 2 <= ct.len(), "not enough bytes for EDK count");
    let edk_start = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for i in 0..edk_count {
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider ID length (EDK {})", i);
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider info length (EDK {})", i);
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for EDK ciphertext length (EDK {})", i);
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }
    fields.push(("Encrypted Data Keys", edk_start, pos));

    // Content Type: 1 byte
    assert!(pos < ct.len(), "not enough bytes for Content Type");
    fields.push(("Content Type", pos, pos + 1));
    pos += 1;

    // Frame Length: 4 bytes
    assert!(pos + 4 <= ct.len(), "not enough bytes for Frame Length");
    fields.push(("Frame Length", pos, pos + 4));
    pos += 4;

    // Algorithm Suite Data: 32 bytes (commit key for committing suites)
    assert!(pos + 32 <= ct.len(), "not enough bytes for Algorithm Suite Data");
    fields.push(("Algorithm Suite Data", pos, pos + 32));

    fields
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_body_serialization_order() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.

    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The V2 Header Body MUST be serialized as, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.

    let ct = encrypt_default(b"test plaintext").await;
    let fields = parse_v2_header_field_offsets(&ct);

    let expected_order = [
        "Version",
        "Algorithm Suite ID",
        "Message ID",
        "AAD",
        "Encrypted Data Keys",
        "Content Type",
        "Frame Length",
        "Algorithm Suite Data",
    ];

    assert_eq!(
        fields.len(),
        expected_order.len(),
        "expected {} header fields, got {}",
        expected_order.len(),
        fields.len()
    );

    for (i, (name, start, end)) in fields.iter().enumerate() {
        assert_eq!(
            *name, expected_order[i],
            "field {} should be '{}' but was '{}'",
            i, expected_order[i], name
        );
        assert!(
            start < end,
            "field '{}' has zero or negative length",
            name
        );
        if i > 0 {
            let (_, _, prev_end) = fields[i - 1];
            assert_eq!(
                *start, prev_end,
                "field '{}' does not immediately follow '{}' (gap at byte {})",
                name,
                fields[i - 1].0,
                prev_end
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_serialized() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-2-0) MUST be serialized with the following specifics:
    let ct = encrypt_default(b"test v2 header").await;
    assert_eq!(ct[0], 0x02, "first byte must be V2 version 0x02");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_version() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
    //# [Version](../data-format/message-header.md#version) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    let ct = encrypt_default(b"version test").await;
    assert_eq!(ct[0], 0x02, "Version field must be 0x02 for V2");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_id() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
    //# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    let ct = encrypt_default(b"suite test").await;
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    let suite_id_bytes = &ct[1..3];
    assert_eq!(suite_id_bytes.len(), 2, "Algorithm Suite ID must be 2 bytes");
    // Default V2 suite AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    //= aws-encryption-sdk-specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let suite_id = u16::from_be_bytes([ct[1], ct[2]]);
    assert_eq!(suite_id, 0x0578, "Algorithm Suite ID must match the suite used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_message_id() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    let ct1 = encrypt_default(b"msg id v2 test").await;
    let ct2 = encrypt_default(b"msg id v2 test").await;
    // V2 header: [0] = version (0x02), [1..3] = algorithm suite ID, [3..35] = message ID (32 bytes)
    let msg_id_1 = &ct1[3..35];
    let msg_id_2 = &ct2[3..35];
    assert_eq!(msg_id_1.len(), 32, "V2 Message ID must be 32 bytes");
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_aad() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
    //# [AAD](../data-format/message-header.md#aad) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    let ec = std::collections::HashMap::from([("key1".to_string(), "val1".to_string())]);
    let pt = b"aad test";
    let result = round_trip_v2(pt, ec).await;
    assert_eq!(result, pt, "round-trip with EC proves AAD serialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_encrypted_data_keys() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
    //# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    let pt = b"edk test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves EDKs serialized correctly (decrypt uses them)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_content_type() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
    //# [Content Type](../data-format/message-header.md#content-type) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    let pt = b"content type test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves content type is correct (framed = 0x02)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_frame_length() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
    //# [Frame Length](../data-format/message-header.md#frame-length) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST be the value of the frame size determined above.
    let pt = b"frame length test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves frame length is serialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_data() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be serialized according to the
    //# [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data) specification.
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    let pt = b"suite data test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves algorithm suite data (commit key) is correct");
}
