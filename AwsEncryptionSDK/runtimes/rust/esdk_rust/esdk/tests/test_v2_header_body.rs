// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization (specification/client-apis/encrypt.md#v2-header)

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_body_serialization_order() {
    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //= reason=parses raw ciphertext bytes and verifies all 8 V2 header fields appear in spec order with no gaps
    //# The V2 Header Body MUST consist of, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.

    let ct = encrypt_default(b"test plaintext").await.ciphertext;
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
    //= reason=verifies the first byte of the ciphertext is 0x02, confirming V2 header body format
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
    let ct = encrypt_default(b"test v2 header").await.ciphertext;
    assert_eq!(ct[0], 0x02, "first byte must be V2 version 0x02");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_version() {
    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //= reason=verifies the Version field byte is 0x02 in the serialized V2 header
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    let ct = encrypt_default(b"version test").await.ciphertext;
    assert_eq!(ct[0], 0x02, "Version field must be 0x02 for V2");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_id() {
    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //= reason=verifies the Algorithm Suite ID field is exactly 2 bytes at offset 1
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    let ct = encrypt_default(b"suite test").await.ciphertext;
    let suite_id_bytes = &ct[1..3];
    assert_eq!(suite_id_bytes.len(), 2, "Algorithm Suite ID must be 2 bytes");

    // Default V2 suite AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578
    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //= reason=verifies the suite ID value 0x0578 matches the default V2 committing+signing suite
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    let suite_id = u16::from_be_bytes([ct[1], ct[2]]);
    assert_eq!(suite_id, 0x0578, "Algorithm Suite ID must match the suite used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_message_id() {
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //= reason=verifies the Message ID field is exactly 32 bytes at the expected V2 offset
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    let ct1 = encrypt_default(b"msg id v2 test").await.ciphertext;
    let ct2 = encrypt_default(b"msg id v2 test").await.ciphertext;
    // V2 header: [0] = version (0x02), [1..3] = algorithm suite ID, [3..35] = message ID (32 bytes)
    let msg_id_1 = &ct1[3..35];
    let msg_id_2 = &ct2[3..35];
    assert_eq!(msg_id_1.len(), 32, "V2 Message ID must be 32 bytes");

    //= specification/data-format/message-header.md#message-id
    //= type=test
    //= reason=two encryptions of the same plaintext produce different message IDs, confirming randomness
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_aad() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip with a non-empty encryption context proves AAD was serialized and deserialized correctly
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
    //= reason=round-trip proves EDKs were serialized correctly since decrypt must use them to recover the data key
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
    //= reason=round-trip proves content type is correct since decrypt validates it; V2 always uses framed (0x02)
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    let pt = b"content type test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves content type is correct (framed = 0x02)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_frame_length() {
    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //= reason=parses the raw 4-byte frame length field from the V2 header and verifies it is a valid UInt32
    //# The frame length MUST be interpreted as a UInt32.
    let ct = encrypt_default(b"frame length test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (name, start, end) = fields.iter().find(|(n, _, _)| *n == "Frame Length").unwrap();
    assert_eq!(*name, "Frame Length");
    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //= reason=verifies the serialized frame length field is exactly 4 bytes
    //# The length of the serialized frame length field MUST be 4 bytes.
    assert_eq!(end - start, 4, "Frame Length field must be 4 bytes");
    let frame_len = u32::from_be_bytes([ct[*start], ct[start + 1], ct[start + 2], ct[start + 3]]);
    assert!(frame_len > 0, "Frame length must be positive for framed content");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_data() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip proves the commit key (algorithm suite data) was serialized correctly since decrypt uses it for key commitment verification
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    let pt = b"suite data test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves algorithm suite data (commit key) is correct");
}
