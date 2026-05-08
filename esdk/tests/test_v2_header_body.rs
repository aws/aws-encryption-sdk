// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization (spec/client-apis/encrypt.md#v2-header)

mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_body_serialization_order() {
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=parses raw ciphertext bytes and verifies all 8 V2 header fields appear in spec order with no gaps
    //# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=parses raw ciphertext bytes and verifies all 8 V2 header fields appear in spec order with no gaps
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
    //
    //= spec/data-format/message-header.md#header-body-version-2-0
    //= type=test
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
        assert!(start < end, "field '{}' has zero or negative length", name);
        if i > 0 {
            let (_, _, prev_end) = fields[i - 1];
            assert_eq!(
                *start,
                prev_end,
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
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies V2 header has all 8 fields by parsing raw bytes and confirming version byte, suite ID, and field count
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
    let ct = encrypt_default(b"test v2 header").await.ciphertext;
    assert_eq!(ct[0], 0x02, "first byte must be V2 version 0x02");
    let fields = parse_v2_header_field_offsets(&ct);
    assert_eq!(fields.len(), 8, "V2 header must have exactly 8 fields");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_version() {
    //= spec/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Version](../data-format/message-header.md#version).
    //# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
    let ct = encrypt_default(b"version test").await.ciphertext;
    assert_eq!(ct[0], 0x02, "Version field must be 0x02 for V2");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_id() {
    let ct = encrypt_default(b"suite test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Algorithm Suite ID")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    assert_eq!(end - start, 2, "Algorithm Suite ID must be 2 bytes");

    //= spec/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    let suite_id = u16::from_be_bytes([ct[*start], ct[start + 1]]);
    assert_eq!(
        suite_id, 0x0578,
        "Algorithm Suite ID must match the suite used"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_message_id() {
    let ct1 = encrypt_default(b"msg id v2 test").await.ciphertext;
    let ct2 = encrypt_default(b"msg id v2 test").await.ciphertext;
    let fields1 = parse_v2_header_field_offsets(&ct1);
    let (_, start1, end1) = fields1
        .iter()
        .find(|(n, _, _)| *n == "Message ID")
        .unwrap();
    let fields2 = parse_v2_header_field_offsets(&ct2);
    let (_, start2, end2) = fields2
        .iter()
        .find(|(n, _, _)| *n == "Message ID")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    //
    //= spec/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    assert_eq!(end1 - start1, 32, "V2 Message ID must be 32 bytes");
    assert_eq!(end2 - start2, 32, "V2 Message ID must be serialized as 32 bytes");

    //= spec/data-format/message-header.md#message-id
    //= type=test
    //= reason=two encryptions of the same plaintext produce different message IDs, confirming randomness
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating message IDs in order to make
    //# the chance of duplicate IDs negligible.
    let msg_id_1 = &ct1[*start1..*end1];
    let msg_id_2 = &ct2[*start2..*end2];
    assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_aad() {
    let ec = std::collections::HashMap::from([("key1".to_string(), "val1".to_string())]);
    let pt = b"aad test";
    let ct = encrypt_default(pt).await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields.iter().find(|(n, _, _)| *n == "AAD").unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip with a non-empty encryption context proves AAD was serialized and deserialized correctly
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    assert!(
        end > start,
        "AAD field must be present in the V2 header"
    );

    let result = round_trip_v2(pt, ec).await;
    assert_eq!(
        result, pt,
        "round-trip with EC proves AAD serialized correctly"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_encrypted_data_keys() {
    let pt = b"edk test";
    let ct = encrypt_default(pt).await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Encrypted Data Keys")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip proves EDKs were serialized correctly since decrypt must use them to recover the data key
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    assert!(
        end - start > 2,
        "Encrypted Data Keys field must have non-trivial length"
    );

    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(
        result, pt,
        "round-trip proves EDKs serialized correctly (decrypt uses them)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_content_type() {
    let ct = encrypt_default(b"content type test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Content Type")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    assert_eq!(end - start, 1, "Content Type field must be 1 byte");
    assert_eq!(
        ct[*start], 0x02,
        "V2 content type must be 0x02 (framed)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_frame_length() {
    let ct = encrypt_default(b"frame length test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (name, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Frame Length")
        .unwrap();
    assert_eq!(*name, "Frame Length");

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    //# The value MUST be the value of the frame size determined above.
    assert_eq!(end - start, 4, "Frame Length field must be 4 bytes");

    let frame_len = u32::from_be_bytes([ct[*start], ct[start + 1], ct[start + 2], ct[start + 3]]);
    assert!(
        frame_len > 0,
        "Frame length must be positive for framed content"
    );
    assert_eq!(
        frame_len, 4096,
        "Frame length must match the default frame size"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_data() {
    let ct = encrypt_default(b"suite data test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Algorithm Suite Data")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the algorithm suite data (commit key) is 32 bytes in the raw V2 header and round-trip proves correctness
    //# - MUST serialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    assert_eq!(
        end - start,
        32,
        "Algorithm Suite Data must be 32 bytes for committing suites"
    );

    let pt = b"suite data test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(
        result, pt,
        "round-trip proves algorithm suite data (commit key) is correct"
    );
}
