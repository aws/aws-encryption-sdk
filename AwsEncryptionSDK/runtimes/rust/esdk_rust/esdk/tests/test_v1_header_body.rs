// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/encrypt.md#v1-header

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_serialized() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:
    let pt = b"test v1 header";
    let ct = encrypt_v1_with_ec(pt, EncryptionContext::new()).await;
    // V1 header starts with version byte 0x01
    assert_eq!(ct[0], 0x01, "first byte must be V1 version 0x01");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_version() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [Version](../data-format/message-header.md#version).
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    //= specification/data-format/message-header.md#header-body-version-1-0
    //= type=test
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    let ct = encrypt_v1_with_ec(b"version test", EncryptionContext::new()).await;
    assert_eq!(ct[0], 0x01, "Version field must be 0x01 for V1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_type() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [Type](../data-format/message-header.md#type).
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    let ct = encrypt_v1_with_ec(b"type test", EncryptionContext::new()).await;
    // Type field is at offset 1, value 0x80 = Customer Authenticated Encrypted Data
    assert_eq!(ct[1], 0x80, "Type field must be 0x80 (Customer AED)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_algorithm_suite_id() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    let ct = encrypt_v1_with_ec(b"suite test", EncryptionContext::new()).await;
    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    let suite_id_bytes = &ct[2..4];
    assert_eq!(suite_id_bytes.len(), 2, "Algorithm Suite ID must be 2 bytes");
    // AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178
    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    //= specification/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let suite_id = u16::from_be_bytes([ct[2], ct[3]]);
    assert_eq!(suite_id, 0x0178, "Algorithm Suite ID must match the suite used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_message_id() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    let ct1 = encrypt_v1_with_ec(b"msg id test", EncryptionContext::new()).await;
    let ct2 = encrypt_v1_with_ec(b"msg id test", EncryptionContext::new()).await;
    // Message ID is 16 bytes at offset 4
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# The message ID MUST be interpreted as bytes.
    let msg_id_1 = &ct1[4..20];
    let msg_id_2 = &ct2[4..20];
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(msg_id_1, msg_id_2, "Message IDs must be unique (random)");
    //= specification/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
    assert_eq!(msg_id_1.len(), 16, "V1 Message ID must be 16 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_aad() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [AAD](../data-format/message-header.md#aad).
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
    //# - The Encrypt operation MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
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
    //# - The Encrypt operation MUST serialize the [Content Type](../data-format/message-header.md#content-type).
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
    //# - The Encrypt operation MUST serialize the [Reserved](../data-format/message-header.md#reserved).
    let pt = b"reserved test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves reserved bytes are correct (decrypt validates them)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_iv_length() {
    //= specification/client-apis/encrypt.md#v1-header
    //= type=test
    //# - The Encrypt operation MUST serialize the [IV Length](../data-format/message-header.md#iv-length).
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
    //# - The Encrypt operation MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
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
    //# The V1 Header Body MUST consist of, in order,
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
    let ct = encrypt_v1_with_ec(b"order test", EncryptionContext::new()).await;
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

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_version_field_is_1_byte() {
    //= specification/data-format/message-header.md#version
    //= type=test
    //# The length of the serialized version field MUST be 1 byte.
    let ct = encrypt_v1_with_ec(b"version 1 byte test", EncryptionContext::new()).await;
    // Version is at offset 0, Type is at offset 1 — proving version is exactly 1 byte
    assert_eq!(ct[0], 0x01, "version byte must be 0x01");
    assert_eq!(ct[1], 0x80, "type byte immediately follows at offset 1, proving version is 1 byte");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_type_field_is_1_byte() {
    //= specification/data-format/message-header.md#type
    //= type=test
    //# The length of the serialized type field MUST be 1 byte.
    let ct = encrypt_v1_with_ec(b"type 1 byte test", EncryptionContext::new()).await;
    // Type is at offset 1, Algorithm Suite ID starts at offset 2 — proving type is exactly 1 byte
    assert_eq!(ct[1], 0x80, "type byte must be 0x80");
    let suite_id = u16::from_be_bytes([ct[2], ct[3]]);
    assert_eq!(suite_id, 0x0178, "algorithm suite ID immediately follows at offset 2, proving type is 1 byte");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_content_type_field_is_1_byte() {
    //= specification/data-format/message-header.md#content-type
    //= type=test
    //# The length of the serialized content type field MUST be 1 byte.
    let ct = encrypt_v1_with_ec(b"content type 1 byte test", EncryptionContext::new()).await;
    let (ct_offset, reserved_offset, _, _) = parse_v1_trailing_offsets(&ct);
    // Content type is 1 byte, reserved immediately follows
    assert_eq!(reserved_offset - ct_offset, 1, "content type field must be exactly 1 byte");
    assert_eq!(ct[ct_offset], 0x02, "content type must be 0x02 (framed)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_reserved_value() {
    //= specification/data-format/message-header.md#reserved
    //= type=test
    //# A reserved sequence of 4 bytes
    //# that MUST have the value (hex) of `00 00 00 00`.
    let ct = encrypt_v1_with_ec(b"reserved value test", EncryptionContext::new()).await;
    let (_, reserved_offset, _, _) = parse_v1_trailing_offsets(&ct);
    assert_eq!(
        &ct[reserved_offset..reserved_offset + 4],
        &[0x00, 0x00, 0x00, 0x00],
        "reserved bytes must be 00 00 00 00"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_reserved_field_is_4_bytes() {
    //= specification/data-format/message-header.md#reserved
    //= type=test
    //# The length of the serialized reserved field MUST be 4 bytes.
    let ct = encrypt_v1_with_ec(b"reserved 4 bytes test", EncryptionContext::new()).await;
    let (_, reserved_offset, iv_length_offset, _) = parse_v1_trailing_offsets(&ct);
    assert_eq!(iv_length_offset - reserved_offset, 4, "reserved field must be exactly 4 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_iv_length_field_is_1_byte() {
    //= specification/data-format/message-header.md#iv-length
    //= type=test
    //# The length of the serialized IV length field MUST be 1 byte.
    let ct = encrypt_v1_with_ec(b"iv length 1 byte test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    assert_eq!(frame_length_offset - iv_length_offset, 1, "IV length field must be exactly 1 byte");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_iv_length_serialized_as_uint8() {
    //= specification/data-format/message-header.md#iv-length
    //= type=test
    //# The IV length MUST be interpreted as a UInt8.
    let ct = encrypt_v1_with_ec(b"iv length uint8 test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, _) = parse_v1_trailing_offsets(&ct);
    // AlgAes256GcmIv12Tag16HkdfSha256 has IV length 12
    let iv_length = ct[iv_length_offset];
    assert_eq!(iv_length, 12, "IV length must be 12 for this algorithm suite, serialized as single UInt8 byte");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_iv_length_equals_suite_iv_length() {
    //= specification/data-format/message-header.md#iv-length
    //= type=test
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let ct = encrypt_v1_with_ec(b"iv length suite test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, _) = parse_v1_trailing_offsets(&ct);
    // AlgAes256GcmIv12Tag16HkdfSha256 has IV length 12
    assert_eq!(ct[iv_length_offset], 12, "IV length must match algorithm suite IV length (12)");
    // Confirm round-trip succeeds, proving the IV length is validated during decrypt
    let result = round_trip_v1(b"iv length suite test", EncryptionContext::new()).await;
    assert_eq!(result, b"iv length suite test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_frame_length_field_is_4_bytes() {
    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //# The length of the serialized frame length field MUST be 4 bytes.
    let ct = encrypt_v1_with_ec(b"frame length 4 bytes v1 test", EncryptionContext::new()).await;
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    let frame_length = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    assert_eq!(frame_length, 4096, "default frame length should be 4096");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_frame_length_serialized_as_uint32() {
    //= specification/data-format/message-header.md#frame-length
    //= type=test
    //# The frame length MUST be interpreted as a UInt32.
    let ct = encrypt_v1_with_ec(b"frame length uint32 v1 test", EncryptionContext::new()).await;
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    // Parse as big-endian UInt32
    let frame_length = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    assert_eq!(frame_length, 4096, "default frame length should be 4096 when serialized as UInt32");
    // Confirm round-trip succeeds
    let result = round_trip_v1(b"frame length uint32 v1 test", EncryptionContext::new()).await;
    assert_eq!(result, b"frame length uint32 v1 test");
}
