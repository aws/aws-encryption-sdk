// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-body-aad.md
//!
//! These tests exercise `body_aad()` directly (via the `__test_internals`
//! hidden module) so assertions hit exact serialized bytes rather than
//! relying on end-to-end round-trips. End-to-end tests remain only where
//! the requirement is about what the CALLER of `body_aad` (i.e.
//! body_encrypt/body_decrypt) must pass in — which can only be verified
//! by observing the full ciphertext.

mod test_helpers;

use aws_esdk::__test_internals::{BodyAADContent, body_aad};
use aws_esdk::EncryptionContext;
use test_helpers::*;

// Known literal values from the spec, repeated here verbatim so the tests
// assert against the specification text (not against the source constants).
const REGULAR_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Frame";
const FINAL_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Final Frame";
const SINGLE_BLOCK_STR: &[u8] = b"AWSKMSEncryptionClient Single Block";

/// Returns the expected body AAD content string length for each variant,
/// used for slicing the serialized output.
fn content_str_bytes(bc: BodyAADContent) -> &'static [u8] {
    match bc {
        BodyAADContent::RegularFrame => REGULAR_FRAME_STR,
        BodyAADContent::FinalFrame => FINAL_FRAME_STR,
        BodyAADContent::SingleBlock => SINGLE_BLOCK_STR,
    }
}

#[test]
fn test_body_aad_structure_ordering() {
    //= specification/data-format/message-body-aad.md#structure
    //= type=test
    //# The message body AAD MUST consist of, in order,
    //# Message ID,
    //# Body AAD Content,
    //# Sequence Number,
    //# and Content Length.
    let msg_id: [u8; 16] = [0x11; 16];
    let seq: u32 = 0xDEAD_BEEF;
    let len: u64 = 0x0102_0304_0506_0708;
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, seq, len, &mut out);

    let content_str = REGULAR_FRAME_STR;
    let expected_total = msg_id.len() + content_str.len() + 4 + 8;
    assert_eq!(out.len(), expected_total, "total length must match structure");

    let mut pos = 0;
    // Message ID first.
    assert_eq!(&out[pos..pos + msg_id.len()], &msg_id, "Message ID must come first");
    pos += msg_id.len();
    // Body AAD Content second.
    assert_eq!(
        &out[pos..pos + content_str.len()],
        content_str,
        "Body AAD Content must follow Message ID"
    );
    pos += content_str.len();
    // Sequence Number third (big-endian u32).
    assert_eq!(
        &out[pos..pos + 4],
        &seq.to_be_bytes(),
        "Sequence Number must follow Body AAD Content"
    );
    pos += 4;
    // Content Length last (big-endian u64).
    assert_eq!(
        &out[pos..pos + 8],
        &len.to_be_bytes(),
        "Content Length must follow Sequence Number"
    );
}

#[test]
fn test_body_aad_message_id_is_copied_verbatim_v1_length() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    //
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let msg_id: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, 1, 100, &mut out);
    assert_eq!(&out[..16], &msg_id, "V1 (16-byte) message ID must be copied verbatim");
}

#[test]
fn test_body_aad_message_id_is_copied_verbatim_v2_length() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    //
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let mut msg_id = [0u8; 32];
    for (i, b) in msg_id.iter_mut().enumerate() {
        *b = i as u8;
    }
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::SingleBlock, 1, 100, &mut out);
    assert_eq!(&out[..32], &msg_id, "V2 (32-byte) message ID must be copied verbatim");
}

#[test]
fn test_body_aad_content_regular_frame_value() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
    let msg_id = [0u8; 16];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + REGULAR_FRAME_STR.len();
    assert_eq!(
        &out[start..end],
        REGULAR_FRAME_STR,
        "regular frame content value must be exactly `AWSKMSEncryptionClient Frame`"
    );
}

#[test]
fn test_body_aad_content_final_frame_value() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
    let msg_id = [0u8; 16];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::FinalFrame, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + FINAL_FRAME_STR.len();
    assert_eq!(
        &out[start..end],
        FINAL_FRAME_STR,
        "final frame content value must be exactly `AWSKMSEncryptionClient Final Frame`"
    );
}

#[test]
fn test_body_aad_content_single_block_value() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - [Nonframed data](message-body.md#nonframed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
    let msg_id = [0u8; 32];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::SingleBlock, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + SINGLE_BLOCK_STR.len();
    assert_eq!(
        &out[start..end],
        SINGLE_BLOCK_STR,
        "nonframed content value must be exactly `AWSKMSEncryptionClient Single Block`"
    );
}

#[test]
fn test_body_aad_content_utf8_encoded() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# The body AAD content value MUST be encoded as UTF-8 bytes.
    // All three literal strings are ASCII (a strict subset of UTF-8); asserting that
    // the serialized bytes equal the `.as_bytes()` of a Rust `str` proves the
    // encoding is UTF-8 by Rust's type-system guarantees.
    let msg_id = [0u8; 16];
    for bc in [
        BodyAADContent::RegularFrame,
        BodyAADContent::FinalFrame,
        BodyAADContent::SingleBlock,
    ] {
        let mut out = Vec::new();
        body_aad(&msg_id, bc, 1, 0, &mut out);
        let start = msg_id.len();
        let expected = content_str_bytes(bc);
        let end = start + expected.len();
        // `expected` is `str::as_bytes()` output — valid UTF-8 by construction.
        std::str::from_utf8(&out[start..end])
            .expect("serialized content bytes must be valid UTF-8");
        assert_eq!(&out[start..end], expected, "{bc:?}: UTF-8 encoded bytes must match literal");
    }
}

#[test]
fn test_body_aad_sequence_number_is_4_bytes_uint32_be() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //# The length of the sequence number field MUST be 4 bytes.
    //
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //# The sequence number field MUST be interpreted as a UInt32.
    let msg_id = [0u8; 16];
    for seq in [0u32, 1, 0x0102_0304, u32::MAX] {
        let mut out = Vec::new();
        body_aad(&msg_id, BodyAADContent::RegularFrame, seq, 0, &mut out);
        let start = msg_id.len() + REGULAR_FRAME_STR.len();
        assert_eq!(
            &out[start..start + 4],
            &seq.to_be_bytes(),
            "seq {seq:#x}: sequence number must be 4-byte UInt32 big-endian"
        );
    }
}

#[test]
fn test_body_aad_content_length_is_8_bytes_uint64_be() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# The length of the content length field MUST be 8 bytes.
    //
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //# The content length field MUST be interpreted as a UInt64.
    let msg_id = [0u8; 16];
    for len in [0u64, 1, 0x0102_0304_0506_0708, u64::MAX] {
        let mut out = Vec::new();
        body_aad(&msg_id, BodyAADContent::RegularFrame, 1, len, &mut out);
        let start = msg_id.len() + REGULAR_FRAME_STR.len() + 4;
        assert_eq!(
            &out[start..start + 8],
            &len.to_be_bytes(),
            "len {len:#x}: content length must be 8-byte UInt64 big-endian"
        );
    }
}

// End-to-end tests below verify caller contracts — requirements that
// constrain the values body_encrypt/body_decrypt pass to body_aad, not
// body_aad's own output. Successful authenticated decryption proves the
// caller used matching values on both sides.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_is_one() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=body_aad takes sequence_number from its caller. This test constructs a nonframed message with sequence number 1 hard-coded in the AAD, then decrypts successfully — proving the real body_encrypt/body_decrypt implementations agree on 1.
    //# For [nonframed data](message-body.md#nonframed-data), the value of this field MUST be `1`.
    let pt = b"nonframed seq num one test";
    let msg = build_nonframed_message(pt);
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(result, pt, "nonframed round-trip proves sequence number in AAD is 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_matches_frame_sequence_number() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=body_aad takes sequence_number from its caller. Observable in ciphertext: each frame's sequence number field is written verbatim into the frame header AND into that frame's AAD. If they disagreed, authenticated decryption would fail.
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    let pt = vec![0xBBu8; 50];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // 50 bytes / 10 per frame = 5 frames; the last is the final frame.
    assert_eq!(frames.len(), 5, "expected 5 frames (4 regular + 1 final)");
    for (i, frame) in frames.iter().enumerate() {
        let expected_seq = (i + 1) as u32;
        assert_eq!(
            frame.0, expected_seq,
            "frame {i}: sequence number field must equal frame's position in sequence"
        );
    }
    // And the whole thing decrypts — proving the AAD used the same sequence numbers.
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves body AAD used matching frame sequence numbers");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_equals_plaintext_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=body_aad takes length from its caller. Nonframed body header contains an explicit 8-byte encrypted-content-length field equal to plaintext length, which must also be what the AAD used for authentication to succeed.
    //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
    let pt = b"nonframed content length test";
    let msg = build_nonframed_message(pt);
    let body = parse_nonframed_body(&msg);
    assert_eq!(
        body.encrypted_content_length, pt.len() as u64,
        "nonframed encrypted content length field must equal plaintext length"
    );
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(result, pt, "decrypt succeeds, proving AAD used the same length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_regular_frame_equals_frame_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=body_aad takes length from its caller. For regular frames, body_encrypt passes frame_length; authenticated decryption succeeds only if the AAD length matched. Observable: each regular frame's encrypted content is exactly frame_length bytes.
    //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of the [frame length](message-header.md#frame-length) field in the message header.
    let pt = vec![0xDDu8; 30];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // Regular frames: is_final=false.
    for (i, frame) in frames.iter().enumerate() {
        if !frame.4 {
            assert_eq!(
                frame.2.len() as u32, frame_length,
                "regular frame {i}: encrypted content length must equal frame_length"
            );
        }
    }
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used frame_length for regular frames");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_final_frame_bounded_by_frame_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=body_aad takes length from its caller. For the final frame, body_encrypt passes the remaining plaintext byte count; the final frame's explicit content_length field and AAD length must match for decryption. Observable: final frame's content_length is in [0, frame_length].
    //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to 0 and less than or equal to the value of the [frame length](message-header.md#frame-length) field in the message header.
    let pt = vec![0xEEu8; 15];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let final_content_len = final_frame_content_length(&ct)
        .expect("ciphertext must contain a final frame");
    assert!(
        final_content_len <= frame_length,
        "final frame content_length ({final_content_len}) must be <= frame_length ({frame_length})"
    );
    // 15 bytes with frame_length=10 -> one regular frame (10) + final frame (5).
    assert_eq!(final_content_len, 5, "final frame should hold remaining 5 bytes");
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used bounded final-frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_framed_equals_per_frame_plaintext() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=body_aad takes length from its caller. For framed data, body_encrypt passes the plaintext length for THIS frame, not the whole message. Observable: sum of per-frame content lengths equals the plaintext length.
    //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes, of the plaintext being encrypted in this frame.
    let pt = vec![0xCCu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // Regular frames contribute frame_length; final frame contributes its remaining bytes.
    let total: usize = frames.iter().map(|f| f.2.len()).sum();
    assert_eq!(
        total, pt.len(),
        "sum of per-frame content lengths must equal plaintext length"
    );
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used per-frame plaintext lengths");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_matches_v1_header() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //= reason=body_aad takes message_id from its caller; for V1 messages the header contains a 16-byte message ID. Successful round-trip proves the AAD message ID length matched the V1 header's.
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let pt = b"v1 message id length test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip proves AAD message ID length matches V1 header (16 bytes)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_matches_v2_header() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //= reason=body_aad takes message_id from its caller; for V2 messages the header contains a 32-byte message ID. Successful decryption of a nonframed V2 message (which uses a 32-byte ID in both the header and AAD) proves the lengths match.
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    let pt = b"v2 message id length test";
    let msg = build_nonframed_message(pt);
    let result = decrypt_nonframed(&msg).await;
    assert_eq!(result, pt, "V2 nonframed round-trip proves AAD message ID length matches V2 header (32 bytes)");
}
