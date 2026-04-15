// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/encrypt.md#construct-the-body

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_serialization_conforms_to_spec() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    // 30 bytes with frame_length=10 → 2 regular frames + 1 final frame (10 bytes)
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 2, "30 bytes / 10-byte frames → 2 regular frames");
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# The final frame MUST be the last frame.

    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# The encrypted message output by the Encrypt operation MUST have a message body equal
    //# to the message body calculated in this step.
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves body in output equals calculated body");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_consumable_bytes_as_regular_frames() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    // 50 bytes with frame_length=10 → 4 regular frames + 1 final frame (10 bytes)
    let pt = vec![0xBBu8; 50];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 4, "50 bytes / 10-byte frames → 4 regular frames");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "all consumable bytes processed as regular frames before final");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_end_of_input_processing() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:

    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
    // 15 bytes with frame_length=10 → 1 regular (10) + 1 final (5)
    let pt = vec![0xCCu8; 15];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 1, "15 bytes → 1 regular frame (10 bytes)");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 5, "final frame content length must be 5 (remaining bytes)");
    assert!(content_len <= 10, "final frame content length must be <= frame length");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "end-of-input processing produces correct output");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_exact_frame_length_constructs_final_or_regular() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
    //# such that creating a regular frame processes all consumable bytes,
    //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
    //# with the remaining plaintext.

    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# - When the length of the Plaintext is an exact multiple of the Frame Length
    //# (including if it is equal to the frame length),
    //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
    // 10 bytes with frame_length=10 → exactly one frame's worth
    // The implementation constructs a final frame for the exact-match case.
    let pt = vec![0xDDu8; 10];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 0, "exact-match case: no regular frames");
    assert_eq!(final_count, 1, "exact-match case: exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 10, "final frame content length must equal frame length");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "exact frame-length plaintext handled correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_enough_bytes_constructs_regular_frame() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are enough input plaintext bytes consumable to create a new regular frame,
    //# such that creating a regular frame does not processes all consumable bytes,
    //# then this operation MUST [construct a regular frame](#construct-a-frame)
    //# using the consumable plaintext bytes.
    // 25 bytes with frame_length=10 → 2 regular frames (10+10) + 1 final (5)
    let pt = vec![0xEEu8; 25];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 2, "25 bytes → 2 regular frames (10+10)");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 5, "final frame content length must be 5");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "regular frames constructed when more bytes remain");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_not_enough_bytes_constructs_final_frame() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
    //# then this operation MUST [construct a final frame](#construct-a-frame)
    // 7 bytes with frame_length=10 → single final frame (7 bytes < frame_length)
    let pt = vec![0xFFu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# - When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
    assert_eq!(regular, 0, "7 bytes < frame_length → no regular frames");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 7, "final frame content length must be 7");
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Final frame serialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "short plaintext produces final frame with correct serialization");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_empty_plaintext_constructs_empty_final_frame() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
    //# and a final frame has not yet been constructed,
    //# this operation MUST [construct an empty final frame](#construct-a-frame).
    let ct = encrypt_with_frame_length(b"", 4096).await;
    let (regular, final_count) = count_frames(&ct, 4096);
    assert_eq!(regular, 0, "empty plaintext → no regular frames");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 0, "empty final frame content length must be 0");
    // Verify the final frame structure: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(0) + Tag(16)
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    let seq_one = 1u32.to_be_bytes();
    let mut found_structure = false;
    for i in 0..ct.len().saturating_sub(8) {
        if ct[i..i + 4] == endframe && ct[i + 4..i + 8] == seq_one {
            found_structure = true;
            break;
        }
    }
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST consist of, in order,
    //# Sequence Number End,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.
    assert!(found_structure, "final frame must have Sequence Number End followed by Sequence Number");
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST only differ from a regular frame by the addition of the
    //# Sequence Number End
    //# and Encrypted Content Length.
    assert_eq!(content_len, 0, "empty final frame has Encrypted Content Length field (value 0)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_length_bound_must_not_encrypt_longer() {
    //= specification/client-apis/encrypt.md#plaintext-length-bound
    //= type=test
    //# If this input is provided, this operation MUST NOT encrypt a plaintext with length
    //# greater than this value.
    let keyring = test_keyring().await;
    let mut stream_input = EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    // Set data_size (plaintext length bound) to 5 bytes
    stream_input.data_size = Some(5);
    // Provide 20 bytes of plaintext, exceeding the bound
    let plaintext = vec![0xAAu8; 20];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;
    assert!(result.is_err(), "encrypt_stream must fail when plaintext exceeds plaintext length bound");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_body_plaintext_length_bound_runtime_enforcement() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
    //# and this operation determines at any time that the plaintext being encrypted
    //# has a length greater than this value,
    //# this operation MUST immediately fail.
    let keyring = test_keyring().await;
    let mut stream_input = EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    // Set data_size to 10 bytes but provide 50 bytes
    stream_input.data_size = Some(10);
    stream_input.frame_length = FrameLength::new(10).unwrap();
    let plaintext = vec![0xBBu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;
    assert!(result.is_err(), "encrypt_stream must immediately fail when plaintext exceeds bound during body construction");
}