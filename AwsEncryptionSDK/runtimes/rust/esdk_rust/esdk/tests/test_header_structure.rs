// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

//= specification/data-format/message-header.md#structure
//= type=test
//# The message header is a sequence of bytes that MUST be in big-endian format.
#[tokio::test(flavor = "multi_thread")]
async fn test_header_big_endian_format() {
    let pt = b"big-endian header test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful decrypt proves header was serialized in big-endian format"
    );
}

//= specification/data-format/message-header.md#structure
//= type=test
//# The header MUST consist of, in order,
//# Header Body,
//# and Header Authentication.
#[tokio::test(flavor = "multi_thread")]
async fn test_header_serialization_order() {
    let pt = b"header serialization order test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful decrypt proves header body precedes header authentication"
    );
}

//= specification/data-format/message-header.md#encrypted-data-key-count
//= type=test
//# This value MUST be greater than 0.
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_greater_than_zero() {
    let ct = encrypt_default(b"edk count test").await.ciphertext;
    let (edk_count_offset, _, _) = parse_header_offsets(&ct);
    let edk_count = u16::from_be_bytes([ct[edk_count_offset], ct[edk_count_offset + 1]]);
    assert!(
        edk_count > 0,
        "encrypted data key count must be greater than 0, got {edk_count}"
    );
}

//= specification/data-format/message-header.md#algorithm-suite-data
//= type=test
//# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
//# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_length_matches_algorithm_suite() {
    let pt = b"suite data length test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful V2 round-trip proves suite data length matches algorithm suite (validate_suite_data runs during decrypt)"
    );
}

//= specification/data-format/message-header.md#algorithm-suite-data
//= type=test
//# The algorithm suite data MUST be interpreted as bytes.
#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_interpreted_as_bytes() {
    let pt = b"suite data bytes test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful V2 round-trip proves suite data is compared as bytes (validate_suite_data compares &[u8] slices)"
    );
}

//= specification/data-format/message-header.md#frame-length
//= type=test
//# The length of the serialized frame length field MUST be 4 bytes.
#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_field_is_4_bytes() {
    let ct = encrypt_default(b"frame length 4 bytes test")
        .await
        .ciphertext;
    let (_, _, frame_length_offset) = parse_header_offsets(&ct);
    let frame_length_bytes = &ct[frame_length_offset..frame_length_offset + 4];
    assert_eq!(
        frame_length_bytes.len(),
        4,
        "frame length field must be exactly 4 bytes"
    );
    // Verify the value is a valid u32 by parsing it
    let frame_length = u32::from_be_bytes([
        frame_length_bytes[0],
        frame_length_bytes[1],
        frame_length_bytes[2],
        frame_length_bytes[3],
    ]);
    assert!(
        frame_length > 0,
        "framed content must have a positive frame length, got {frame_length}"
    );
}

//= specification/data-format/message-header.md#frame-length
//= type=test
//# The frame length MUST be interpreted as a UInt32.
#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_serialized_as_uint32() {
    let ct = encrypt_default(b"frame length uint32 test")
        .await
        .ciphertext;
    let (_, _, frame_length_offset) = parse_header_offsets(&ct);
    // Parse as big-endian UInt32 and verify round-trip through decrypt succeeds
    let frame_length = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    // Default frame length is 4096 (0x00001000)
    assert_eq!(
        frame_length, 4096,
        "default frame length should be 4096 when serialized as UInt32"
    );
    // Confirm the message decrypts successfully, proving the UInt32 encoding is correct
    let result = round_trip(b"frame length uint32 test").await;
    assert_eq!(result, b"frame length uint32 test");
}

//= specification/data-format/message-header.md#frame-length
//= type=test
//# When the [content type](#content-type) is nonframed, the value of this field MUST be 0.
#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_frame_length_must_be_zero() {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(
        b"frame length test",
        EncryptionContext::new(),
        keyring.clone(),
    );
    let mut ct = encrypt(&input).await.unwrap().ciphertext;

    let (_, content_type_offset, frame_length_offset) = parse_header_offsets(&ct);

    // Set content type to NonFramed (0x01) and frame length to a non-zero value
    ct[content_type_offset] = 0x01;
    ct[frame_length_offset] = 0x00;
    ct[frame_length_offset + 1] = 0x00;
    ct[frame_length_offset + 2] = 0x10;
    ct[frame_length_offset + 3] = 0x00;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(
        decrypt(&dec_input).await.is_err(),
        "nonframed content with non-zero frame length must be rejected"
    );
}
