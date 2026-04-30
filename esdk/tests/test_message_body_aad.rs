// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-body-aad.md

mod test_helpers;

use test_helpers::*;
use aws_esdk::EncryptionContext;

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_structure_ordering() {
    //= specification/data-format/message-body-aad.md#structure
    //= type=test
    //# The message body AAD MUST consist of, in order,
    //# Message ID,
    //# Body AAD Content,
    //# Sequence Number,
    //# and Content Length.
    let pt = b"body aad structure test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves body AAD fields are in correct order"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_from_header() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    let pt = b"message id from header test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves body AAD uses the message ID from the header"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_utf8_encoding() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# The body AAD content value MUST be encoded as UTF-8 bytes.
    let pt = b"utf8 encoding test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves body AAD content is correctly UTF-8 encoded"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_regular_frame_value() {
    // Multi-frame: 30 bytes with frame_length=10 produces regular frames
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data)
    //# MUST use the value `AWSKMSEncryptionClient Frame`.
    let pt = vec![0xAAu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves regular frame body AAD content value is correct"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_final_frame_value() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data)
    //# MUST use the value `AWSKMSEncryptionClient Final Frame`.
    let pt = b"final frame aad test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame body AAD content value is correct"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_length() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //# The length of the sequence number field MUST be 4 bytes.
    let pt = b"seq num length test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves sequence number field is 4 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_uint32() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //# The sequence number field MUST be interpreted as a UInt32.
    let pt = b"seq num uint32 test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves sequence number is interpreted as UInt32"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_value() {
    // Multi-frame: each frame's AAD must use the correct sequence number
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    let pt = vec![0xBBu8; 50];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame round-trip proves per-frame sequence numbers in AAD are correct"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_8_bytes() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# The length of the content length field MUST be 8 bytes.
    let pt = b"content length 8 bytes test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves content length field is 8 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_uint64() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# The content length field MUST be interpreted as a UInt64.
    let pt = b"content length uint64 test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result, pt,
        "round-trip proves content length is interpreted as UInt64"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_framed_value() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes,
    //# of the plaintext being encrypted in this frame.
    let pt = vec![0xCCu8; 25];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves content length in AAD equals plaintext length per frame"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_regular_frame_equals_frame_length() {
    // 30 bytes with frame_length=10 → regular frames have content_length=10
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of
    //# the [frame length](message-header.md#frame-length) field in the message header.
    let pt = vec![0xDDu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves regular frame content length equals frame length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_final_frame_bounded() {
    // 15 bytes with frame_length=10 → final frame has 5 bytes (0 < 5 <= 10)
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to
    //# 0 and less than or equal to the value of the [frame length](message-header.md#frame-length)
    //# field in the message header.
    let pt = vec![0xEEu8; 15];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame content length is bounded by frame length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_nonframed_single_block() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //= reason=builds and decrypts a nonframed message; the AAD uses "AWSKMSEncryptionClient Single Block" — if wrong, authenticated decryption would fail
    //# - [Nonframed data](message-body.md#nonframed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
    let pt = b"nonframed single block aad test";
    let msg = build_nonframed_message(pt);
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(
        result, pt,
        "nonframed round-trip proves body AAD content is 'AWSKMSEncryptionClient Single Block'"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_is_one() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=builds and decrypts a nonframed message; the AAD uses sequence number 1 — if wrong, authenticated decryption would fail
    //# For [nonframed data](message-body.md#nonframed-data), the value of this field MUST be `1`.
    let pt = b"nonframed seq num one test";
    let msg = build_nonframed_message(pt);
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(
        result, pt,
        "nonframed round-trip proves sequence number in AAD is 1"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_equals_plaintext() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=builds and decrypts a nonframed message; the AAD content length equals the plaintext length — if wrong, authenticated decryption would fail
    //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes,
    //# of the plaintext data provided to the algorithm for encryption.
    let pt = b"nonframed content length test";
    let msg = build_nonframed_message(pt);
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(
        result, pt,
        "nonframed round-trip proves content length in AAD equals plaintext length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_matches_header_version() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //= reason=builds and decrypts a nonframed V2 message (32-byte message ID); if the AAD message ID length didn't match the header version's message ID length, authenticated decryption would fail
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let pt = b"message id length test";
    let msg = build_nonframed_message(pt);
    // V2 message ID is 32 bytes; the AAD uses the same 32-byte message ID from the header
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(
        result, pt,
        "nonframed V2 round-trip proves message ID length in AAD matches header version (32 bytes)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_v1() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //= reason=V1 round-trip uses a 16-byte message ID; if the AAD message ID length didn't match the V1 header's 16-byte message ID, authenticated decryption would fail
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let pt = b"v1 message id length test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(
        result, pt,
        "V1 round-trip proves message ID length in AAD matches V1 header version (16 bytes)"
    );
}
