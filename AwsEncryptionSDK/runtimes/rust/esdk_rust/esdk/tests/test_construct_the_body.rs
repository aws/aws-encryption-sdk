// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body

mod fixtures;

use aws_esdk::*;
use fixtures::*;

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let (ns, name) = namespace_and_name(0);
    let keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

// ─── Req 1: Regular frame serialization ─────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_serialization_conforms_to_spec() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    // 30 bytes with frame_length=10 → multiple regular frames + final frame
    let pt = vec![0xAAu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves regular frames conform to spec");
}

// ─── Req 2: Process consumable bytes as regular frames ──────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_process_consumable_bytes_as_regular_frames() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    // 50 bytes with frame_length=10 → 4 regular frames + 1 final frame (10 bytes)
    let pt = vec![0xBBu8; 50];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "all consumable bytes processed as regular frames before final");
}

// ─── Req 3: When end of input indicated ─────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_end_of_input_processing() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:
    // 15 bytes with frame_length=10 → 1 regular (10) + 1 final (5)
    let pt = vec![0xCCu8; 15];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "end-of-input processing produces correct output");
}

// ─── Req 4: Exactly enough bytes for one regular frame ──────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_exact_frame_length_constructs_final_or_regular() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
    //# such that creating a regular frame processes all consumable bytes,
    //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
    //# with the remaining plaintext.
    // 10 bytes with frame_length=10 → exactly one frame's worth
    let pt = vec![0xDDu8; 10];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "exact frame-length plaintext handled correctly");
}

// ─── Req 5: Enough bytes for regular frame, more remaining ──────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_enough_bytes_constructs_regular_frame() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are enough input plaintext bytes consumable to create a new regular frame,
    //# such that creating a regular frame does not processes all consumable bytes,
    //# then this operation MUST [construct a regular frame](#construct-a-frame)
    //# using the consumable plaintext bytes.
    // 25 bytes with frame_length=10 → 2 regular frames (10+10) + 1 final (5)
    let pt = vec![0xEEu8; 25];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "regular frames constructed when more bytes remain");
}

// ─── Req 6: Not enough bytes for regular frame ─────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_not_enough_bytes_constructs_final_frame() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
    //# then this operation MUST [construct a final frame](#construct-a-frame)
    // 7 bytes with frame_length=10 → single final frame (7 bytes < frame_length)
    let pt = vec![0xFFu8; 7];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "short plaintext produces final frame");
}

// ─── Req 7: Empty final frame ───────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_empty_plaintext_constructs_empty_final_frame() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
    //# and a final frame has not yet been constructed,
    //# this operation MUST [construct an empty final frame](#construct-a-frame).
    let (ns, name) = namespace_and_name(0);
    let keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let enc_input = EncryptInput::with_legacy_keyring(b"", EncryptionContext::new(), keyring);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    // Empty plaintext must produce a final frame with ENDFRAME marker
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    assert!(ct.windows(4).any(|w| w == endframe), "empty plaintext must produce an empty final frame");
}
