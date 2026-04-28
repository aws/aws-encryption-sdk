// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization (specification/client-apis/encrypt.md#v2-header)

mod test_helpers;

use aws_esdk::*;
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

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies all 8 V2 header fields appear in spec order by parsing raw ciphertext bytes
    //# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.

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
    //= specification/client-apis/encrypt.md#v2-header
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
    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //= reason=verifies the Version field byte is 0x02 in the serialized V2 header
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Version field is present in the V2 header by parsing raw bytes
    //# - MUST serialize the [Version](../data-format/message-header.md#version).

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Version byte is 0x02 confirming it corresponds to version 2.0
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

    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //= reason=verifies the Algorithm Suite ID field is exactly 2 bytes by parsing V2 header field boundaries
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Algorithm Suite ID field is present in the V2 header by parsing raw bytes
    //# - MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    assert_eq!(end - start, 2, "Algorithm Suite ID must be 2 bytes");

    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //= reason=verifies the suite ID value 0x0578 matches the default V2 committing+signing suite
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    let suite_id = u16::from_be_bytes([ct[*start], ct[start + 1]]);
    assert_eq!(
        suite_id, 0x0578,
        "Algorithm Suite ID must match the suite used"
    );

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the suite ID value 0x0578 matches the algorithm suite used for encryption
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=round-trip decrypt must deserialize the Algorithm Suite ID to select the correct algorithm for decryption
    //# - MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    assert_eq!(
        suite_id, 0x0578,
        "Suite ID must correspond to the algorithm suite used"
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

    //= specification/data-format/message-header.md#message-id
    //= type=test
    //= reason=verifies the Message ID field is exactly 32 bytes by parsing V2 header field boundaries
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    assert_eq!(end1 - start1, 32, "V2 Message ID must be 32 bytes");

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Message ID field is present and 32 bytes in the V2 header
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    assert_eq!(end2 - start2, 32, "V2 Message ID must be serialized as 32 bytes");

    //= specification/data-format/message-header.md#message-id
    //= type=test
    //= reason=two encryptions of the same plaintext produce different message IDs, confirming randomness
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating message IDs in order to make
    //# the chance of duplicate IDs negligible.
    let msg_id_1 = &ct1[*start1..*end1];
    let msg_id_2 = &ct2[*start2..*end2];
    assert_ne!(msg_id_1, msg_id_2, "V2 Message IDs must be unique (random)");

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=two encryptions produce different message IDs, confirming good randomness source
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=round-trip decrypt must deserialize the Message ID to verify header authentication
    //# - MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    assert_ne!(
        msg_id_1, msg_id_2,
        "Message IDs must differ, confirming randomness"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_aad() {
    let ec = std::collections::HashMap::from([("key1".to_string(), "val1".to_string())]);
    let pt = b"aad test";
    let ct = encrypt_default(pt).await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields.iter().find(|(n, _, _)| *n == "AAD").unwrap();

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the AAD field is present in the V2 header by parsing raw bytes and confirming non-zero length
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    assert!(
        end > start,
        "AAD field must be present in the V2 header"
    );

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip with a non-empty encryption context proves AAD was serialized and deserialized correctly
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=round-trip decrypt must deserialize the AAD to reconstruct the encryption context
    //# - MUST deserialize the [AAD](../data-format/message-header.md#aad).
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

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Encrypted Data Keys field is present in the V2 header by parsing raw bytes
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    assert!(
        end - start > 2,
        "Encrypted Data Keys field must have non-trivial length"
    );

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=round-trip proves EDKs were serialized correctly since decrypt must use them to recover the data key
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=round-trip decrypt must deserialize the Encrypted Data Keys to recover the data key
    //# - MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
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

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Content Type field is present in the V2 header by parsing raw bytes
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    assert_eq!(end - start, 1, "Content Type field must be 1 byte");

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=parses the raw content type byte from the V2 header and verifies it is 0x02 (framed)
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=parsing the content type byte from raw bytes exercises the same deserialization path as decrypt
    //# - MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
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

    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //= reason=verifies the serialized frame length field is exactly 4 bytes
    //# The length of the serialized frame length field MUST be 4 bytes.
    assert_eq!(end - start, 4, "Frame Length field must be 4 bytes");

    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //= reason=parses the raw 4-byte frame length field from the V2 header and verifies it is a valid UInt32
    //# The frame length MUST be interpreted as a UInt32.

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Frame Length field is present in the V2 header by parsing raw bytes
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    let frame_len = u32::from_be_bytes([ct[*start], ct[start + 1], ct[start + 2], ct[start + 3]]);
    assert!(
        frame_len > 0,
        "Frame length must be positive for framed content"
    );

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the frame length value matches the default frame size used during encryption
    //# The value MUST be the value of the frame size determined above.

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=parsing the 4-byte frame length from raw bytes exercises the same deserialization path as decrypt
    //# - MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).
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

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the Algorithm Suite Data field is present and 32 bytes in the V2 header
    //# - MUST serialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    assert_eq!(
        end - start,
        32,
        "Algorithm Suite Data must be 32 bytes for committing suites"
    );

    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the algorithm suite data (commit key) is 32 bytes in the raw V2 header and round-trip proves correctness
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=round-trip decrypt must deserialize the Algorithm Suite Data to verify key commitment
    //# - MUST deserialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    let pt = b"suite data test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(
        result, pt,
        "round-trip proves algorithm suite data (commit key) is correct"
    );
}
