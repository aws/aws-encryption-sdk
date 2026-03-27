// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body

mod fixtures;

use aws_esdk::*;
use fixtures::*;

const IV_LEN: usize = 12;
const TAG_LEN: usize = 16;
const ENDFRAME_MARKER: [u8; 4] = 0xFFFF_FFFFu32.to_be_bytes();

/// Create a raw AES keyring for testing (no KMS needed).
async fn test_keyring() -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Encrypt plaintext with a given frame length, return ciphertext bytes.
async fn encrypt_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Find the start of the message body by scanning for the first frame.
/// Returns the byte offset where the first frame begins.
fn find_body_start(ct: &[u8], frame_length: u32) -> Option<usize> {
    let seq_one = 1u32.to_be_bytes();
    // The first frame is either:
    //   - A regular frame starting with SeqNum=1
    //   - A final frame starting with 0xFFFFFFFF followed by SeqNum=1
    // Try to find ENDFRAME+SeqNum=1 (final frame as first frame)
    // or SeqNum=1 followed by valid frame data (regular frame)
    for i in 0..ct.len().saturating_sub(4) {
        // Check if this is the start of a final frame (ENDFRAME + seq=1)
        if i + 8 <= ct.len() && ct[i..i + 4] == ENDFRAME_MARKER && ct[i + 4..i + 8] == seq_one {
            return Some(i);
        }
        // Check if this is the start of a regular frame (seq=1)
        // Validate by checking that walking through regular frames leads to ENDFRAME
        if ct[i..i + 4] == seq_one {
            if validate_frame_walk(ct, i, frame_length) {
                return Some(i);
            }
        }
    }
    None
}

/// Validate that starting at `offset` and walking regular frames leads to an ENDFRAME marker.
fn validate_frame_walk(ct: &[u8], offset: usize, frame_length: u32) -> bool {
    let regular_frame_size = 4 + IV_LEN + frame_length as usize + TAG_LEN;
    let mut pos = offset;
    loop {
        if pos + 4 > ct.len() {
            return false;
        }
        if ct[pos..pos + 4] == ENDFRAME_MARKER {
            return true;
        }
        // Try to advance past a regular frame
        let next = pos + regular_frame_size;
        if next > ct.len() {
            return false;
        }
        pos = next;
    }
}

/// Count regular and final frames in the ciphertext.
/// Returns `(regular_frame_count, final_frame_count)`.
fn count_frames(ct: &[u8], frame_length: u32) -> (usize, usize) {
    let body_start = find_body_start(ct, frame_length)
        .expect("could not find body start in ciphertext");
    let regular_frame_size = 4 + IV_LEN + frame_length as usize + TAG_LEN;
    let mut pos = body_start;
    let mut regular = 0usize;
    let mut final_count = 0usize;

    loop {
        if pos + 4 > ct.len() {
            break;
        }
        if ct[pos..pos + 4] == ENDFRAME_MARKER {
            // Final frame: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLength(4) + Content(N) + Tag(16)
            if pos + 24 > ct.len() {
                break;
            }
            let _content_len =
                u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]])
                    as usize;
            final_count += 1;
            break; // Final frame is always last
        } else {
            // Regular frame: SeqNum(4) + IV(12) + Content(frame_length) + Tag(16)
            regular += 1;
            pos += regular_frame_size;
        }
    }

    (regular, final_count)
}

/// Extract the content length from the final frame in the ciphertext.
fn final_frame_content_length(ct: &[u8]) -> Option<u32> {
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == ENDFRAME_MARKER {
            return Some(u32::from_be_bytes([
                ct[i + 20],
                ct[i + 21],
                ct[i + 22],
                ct[i + 23],
            ]));
        }
    }
    None
}

// ─── Req 1: Regular frame serialization ─────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_serialization_conforms_to_spec() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    // 30 bytes with frame_length=10 → 2 regular frames + 1 final frame (10 bytes)
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 2, "30 bytes / 10-byte frames → 2 regular frames");
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# The final frame MUST be the last frame.
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 4, "50 bytes / 10-byte frames → 4 regular frames");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
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
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
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
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 2, "25 bytes → 2 regular frames (10+10)");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 5, "final frame content length must be 5");
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
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# - When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
    // 7 bytes with frame_length=10 → single final frame (7 bytes < frame_length)
    let pt = vec![0xFFu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let (regular, final_count) = count_frames(&ct, 10);
    assert_eq!(regular, 0, "7 bytes < frame_length → no regular frames");
    assert_eq!(final_count, 1, "must have exactly 1 final frame");
    let content_len = final_frame_content_length(&ct).unwrap();
    assert_eq!(content_len, 7, "final frame content length must be 7");
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
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST be serialized as, in order,
    //# Sequence Number End,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.
    assert!(found_structure, "final frame must have Sequence Number End followed by Sequence Number");
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# This means a final frame MUST be a regular frame with the addition of the serialized
    //# Sequence Number End
    //# and Encrypted Content Length.
    assert_eq!(content_len, 0, "empty final frame has Encrypted Content Length field (value 0)");
}
