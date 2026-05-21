// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#construct-the-body

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_process_consumable_bytes_as_regular_frames() {
    // 50 bytes / frame_length=10 → 4 regular + 1 final (10).
    let pt = vec![0xBBu8; 50];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    assert_eq!(regular, 4);
    assert_eq!(final_count, 1);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //= reason=round-trip proves encrypt and decrypt agree on the body bytes
    //# The encrypted message output by the Encrypt operation MUST have a message body equal
    //# to the message body calculated in this step.
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_end_of_input_processing() {
    // 15 bytes / frame_length=10 → 1 regular (10) + 1 final (5).
    let pt = vec![0xCCu8; 15];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:
    assert_eq!(regular, 1);
    assert_eq!(final_count, 1);
    assert_eq!(final_frame_content_length(&ct).unwrap(), 5);

    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_exact_frame_length_constructs_final_or_regular() {
    // 10 bytes / frame_length=10 → exactly one frame (final, in this impl).
    let pt = vec![0xDDu8; 10];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
    //# such that creating a regular frame processes all consumable bytes,
    //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
    //# with the remaining plaintext.
    assert_eq!(regular + final_count, 1);
    assert_eq!(final_frame_content_length(&ct).unwrap(), 10);

    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_enough_bytes_constructs_regular_frame() {
    // 25 bytes / frame_length=10 → 2 regular + 1 final (5).
    let pt = vec![0xEEu8; 25];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are enough input plaintext bytes consumable to create a new regular frame,
    //# such that creating a regular frame does not processes all consumable bytes,
    //# then this operation MUST [construct a regular frame](#construct-a-frame)
    //# using the consumable plaintext bytes.
    assert_eq!(regular, 2);
    assert_eq!(final_count, 1);
    assert_eq!(final_frame_content_length(&ct).unwrap(), 5);

    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_not_enough_bytes_constructs_final_frame() {
    // 7 bytes / frame_length=10 → single final frame.
    let pt = vec![0xFFu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
    //# then this operation MUST [construct a final frame](#construct-a-frame)
    assert_eq!(regular, 0);
    assert_eq!(final_count, 1);
    assert_eq!(final_frame_content_length(&ct).unwrap(), 7);

    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_empty_plaintext_constructs_empty_final_frame() {
    let ct = encrypt_with_frame_length(b"", 4096).await;
    let (regular, final_count) = count_frames(&ct, 4096);

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
    //# and a final frame has not yet been constructed,
    //# this operation MUST [construct an empty final frame](#construct-a-frame).
    assert_eq!(regular, 0);
    assert_eq!(final_count, 1);
    assert_eq!(final_frame_content_length(&ct).unwrap(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_length_bound_must_not_encrypt_longer() {
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.data_size = Some(5);
    let plaintext = vec![0xAAu8; 20];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;

    //= spec/client-apis/encrypt.md#plaintext-length-bound
    //= type=test
    //# If this input is provided, this operation MUST NOT encrypt a plaintext with length
    //# greater than this value.
    let err = result.expect_err("must reject");
    assert!(matches!(err.kind, ErrorKind::ValidationError), "got {:?}", err.kind);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_body_plaintext_length_bound_runtime_enforcement() {
    // Bound 10, plaintext 50, frame 10: encoder must fail mid-stream.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.data_size = Some(10);
    stream_input.frame_length = FrameLength::new(10).unwrap();
    let plaintext = vec![0xBBu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;

    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
    //# and this operation determines at any time that the plaintext being encrypted
    //# has a length greater than this value,
    //# this operation MUST immediately fail.
    let err = result.expect_err("must fail mid-stream");
    assert!(matches!(err.kind, ErrorKind::ValidationError), "got {:?}", err.kind);
}
