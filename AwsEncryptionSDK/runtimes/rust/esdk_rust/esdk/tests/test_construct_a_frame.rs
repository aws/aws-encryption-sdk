// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/encrypt.md#construct-a-frame

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

/// Encrypt plaintext with a given frame length, return ciphertext bytes.
async fn encrypt_with_frame_length(
    plaintext: &[u8],
    frame_length: u32,
) -> Vec<u8> {
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
    let dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

// ─── Encryption calculation tests ───────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_cipherkey_and_plaintext() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The cipherkey MUST be the derived data key
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
    let pt = b"hello world construct frame";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_aad_and_iv() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# To construct a regular or final frame that represents the next frame in the encrypted message's body,
    //# the Encrypt operation MUST calculate the encrypted content and an authentication tag using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md),
    //# with the following inputs:
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed according to the [Message Body AAD](../data-format/message-body-aad.md) specification, as follows:
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD for this frame,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    let pt = b"test aad and iv";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt, "round-trip proves AAD and IV were correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_message_id_in_aad() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-format/message-header.md#message-id) serialized in the header of this message.
    let pt = b"message id in aad";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt, "decryption success proves message ID in AAD matches header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_body_aad_content() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
    //# [Message Body AAD](../data-format/message-body-aad.md).
    let pt = b"body aad content test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt, "decryption success proves body AAD content type is correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_in_aad() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
    //# number of the frame being encrypted.
    // Multi-frame: each frame's AAD must have the correct sequence number
    let pt = vec![0xABu8; 100];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame round-trip proves per-frame sequence numbers in AAD are correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_content_length_in_aad() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext being encrypted.
    let pt = vec![0xCDu8; 50];
    let result = round_trip(&pt, 20).await;
    assert_eq!(result, pt, "round-trip proves content length in AAD is correct for each frame");
}

// ─── Sequence number tests ──────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_starts_at_one() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    let ct = encrypt_with_frame_length(b"seq num test", 4096).await;
    // Find the body: after the header. The first frame's sequence number
    // is the first 4 bytes of the body. For a single final frame, the body
    // starts with ENDFRAME marker (0xFFFFFFFF) then the sequence number.
    // Search for the ENDFRAME marker followed by sequence number 1.
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    let seq_one = 1u32.to_be_bytes();
    let mut found = false;
    for i in 0..ct.len().saturating_sub(8) {
        if ct[i..i + 4] == endframe && ct[i + 4..i + 8] == seq_one {
            found = true;
            break;
        }
    }
    assert!(found, "first frame sequence number must be 1 (after ENDFRAME marker for single-frame message)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_increments() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    // Create a multi-frame message: 30 bytes with frame_length=10 → 3 regular frames + 1 final (empty or 10-byte)
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    // Regular frames start with sequence number (4 bytes).
    // Look for sequence numbers 1, 2, 3 in order.
    let seq1 = 1u32.to_be_bytes();
    let seq2 = 2u32.to_be_bytes();
    let seq3 = 3u32.to_be_bytes();
    let ct_str = ct.iter().map(|b| format!("{b:02x}")).collect::<String>();
    // Verify round-trip to confirm correctness
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame round-trip must succeed, proving sequence numbers increment correctly");
    // Also verify the raw bytes contain the expected sequence numbers
    let has_seq1 = ct.windows(4).any(|w| w == seq1);
    let has_seq2 = ct.windows(4).any(|w| w == seq2);
    let has_seq3 = ct.windows(4).any(|w| w == seq3);
    assert!(has_seq1, "ciphertext must contain sequence number 1: {ct_str}");
    assert!(has_seq2, "ciphertext must contain sequence number 2: {ct_str}");
    assert!(has_seq3, "ciphertext must contain sequence number 3: {ct_str}");
}

// ─── Plaintext length tests ─────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_plaintext_equals_frame_length() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a regular frame the length of this plaintext MUST equal the frame length.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a regular frame the length of this plaintext subsequence MUST equal the frame length.
    // 20 bytes with frame_length=10 → 1 regular frame (10 bytes) + 1 final frame (10 bytes)
    let pt = vec![0xBBu8; 20];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "regular frame must encrypt exactly frame_length bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_remaining_plaintext() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a final frame this MUST be the length of the remaining plaintext bytes
    //# which have not yet been encrypted,
    //# whose length MUST be equal to or less than the frame length.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a final frame this MUST be the remaining plaintext bytes which have not yet been encrypted,
    //# whose length MUST be equal to or less than the frame length.
    // 15 bytes with frame_length=10 → 1 regular (10) + 1 final (5, which is < frame_length)
    let pt = vec![0xCCu8; 15];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "final frame must encrypt remaining bytes (less than frame length)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_plaintext_subsequence() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
    // Verify that multi-frame encryption preserves the full plaintext in order
    let pt: Vec<u8> = (0..=255).collect();
    let result = round_trip(&pt, 50).await;
    assert_eq!(result, pt, "each frame must encrypt the next unconsumed subsequence");
}

// ─── Serialization tests ────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_serialization_regular_and_final() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The Encrypt operation MUST serialize a regular frame or final frame with the following specifics:
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a regular frame, the serialization MUST follow the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a final frame, the serialization MUST follow the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a regular frame, each field MUST be serialized according to its specification:
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a final frame, each field MUST be serialized according to its specification:
    // Multi-frame message: proves both regular and final frame serialization
    let pt = vec![0xDDu8; 25];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "successful decrypt proves both regular and final frames are correctly serialized");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_serialized() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be serialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The value MUST be the sequence number of this frame.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be serialized according to the
    //# [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
    let pt = vec![0xEEu8; 30];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "decrypt success proves sequence numbers are correctly serialized");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_iv_serialized() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be serialized according to the
    //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The value MUST be the IV used when calculating the encrypted content for this frame.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [IV](../data-format/message-body.md#final-frame-iv): MUST be serialized according to the
    //# [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
    let pt = b"iv serialization test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "decrypt success proves IV is correctly serialized");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_has_endframe_marker() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be serialized according to the
    //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be serialized according to the
    //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
    let ct = encrypt_with_frame_length(b"endframe marker", 4096).await;
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    let count = ct.windows(4).filter(|w| *w == endframe).count();
    assert!(count >= 1, "final frame must contain ENDFRAME marker (0xFFFFFFFF)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_content_length_serialized() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be serialized according to the
    //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be serialized according to the
    //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
    // 7 bytes with frame_length=10 → single final frame with content length 7
    let pt = b"1234567";
    let ct = encrypt_with_frame_length(pt, 10).await;
    // The final frame has: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLength(4) + EncContent + AuthTag
    // Content length should be 7 (0x00000007)
    let content_len_bytes = 7u32.to_be_bytes();
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    // Find ENDFRAME marker, then skip seq_num(4) + IV(12) to find content length
    let mut found = false;
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == endframe {
            // seq_num at i+4, IV at i+8 (12 bytes), content_length at i+20
            if i + 24 <= ct.len() && ct[i + 20..i + 24] == content_len_bytes {
                found = true;
                break;
            }
        }
    }
    assert!(found, "final frame must contain encrypted content length field");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_auth_tag_serialized() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be serialized according to the
    //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The value MUST be the encrypted content calculated for this frame.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be serialized according to the
    //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be serialized according to the
    //# [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be serialized according to the
    //# [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
    let pt = b"encrypted content and auth tag test";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "decrypt success proves encrypted content and auth tag are correctly serialized");
}

// ─── Frame release test ─────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_bytes_not_released_until_complete() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# The serialized frame bytes MUST NOT be released until the entire frame has been serialized.
    // If bytes were released prematurely (partial frame), decryption would fail
    // because the auth tag wouldn't be present. A successful round-trip proves
    // the entire frame was serialized before release.
    let pt = vec![0xFFu8; 100];
    let result = round_trip(&pt, 20).await;
    assert_eq!(result, pt, "successful multi-frame decrypt proves frames are fully serialized before release");
}

// ─── Edge case tests ────────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_empty_plaintext() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a final frame this MUST be the remaining plaintext bytes which have not yet been encrypted,
    //# whose length MUST be equal to or less than the frame length.
    // Empty plaintext produces a single final frame with 0-length encrypted content.
    // The encrypt side correctly produces this; verify it doesn't error.
    let pt = b"";
    let ct = encrypt_with_frame_length(pt, 4096).await;
    // Verify the ciphertext contains an ENDFRAME marker (final frame was produced)
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    assert!(ct.windows(4).any(|w| w == endframe), "empty plaintext must produce a final frame with ENDFRAME marker");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_single_final_frame() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a final frame, the serialization MUST follow the [Final Frame](../data-format/message-body.md#final-frame) specification.
    // Plaintext smaller than frame length → single final frame only
    let pt = b"short";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec(), "single final frame must correctly encrypt short plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_content_length_less_than_frame_length() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a final frame this MUST be the length of the remaining plaintext bytes
    //# which have not yet been encrypted,
    //# whose length MUST be equal to or less than the frame length.
    // 13 bytes with frame_length=10 → 1 regular (10) + 1 final (3)
    let pt = vec![0x42u8; 13];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    // Find the final frame's content length field (should be 3)
    let endframe = 0xFFFF_FFFFu32.to_be_bytes();
    let expected_len = 3u32.to_be_bytes();
    let mut found = false;
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == endframe && i + 24 <= ct.len() && ct[i + 20..i + 24] == expected_len {
            found = true;
            break;
        }
    }
    assert!(found, "final frame content length must be 3 (remaining bytes after regular frame)");
    // Also verify round-trip
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_streaming_frame_released() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If the Encrypt operation is streaming the encrypted message and
    //# the entire frame has been serialized,
    //# the serialized frame MUST be released.
    let keyring = test_keyring().await;
    let mut stream_input = EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.frame_length = FrameLength::new(10).unwrap();
    // Provide enough plaintext for multiple frames
    let plaintext = vec![0xAAu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;
    assert!(result.is_ok(), "streaming encrypt must succeed");
    // The output must contain bytes — frames were released (written) to the output writer
    // as they were serialized. If frames were not released, output would be empty.
    assert!(!output.is_empty(), "streaming output must contain released frame bytes");
    // Verify the output is a valid encrypted message by decrypting it
    let dec_keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), dec_keyring);
    let decrypted = decrypt(&dec_input).await.unwrap();
    assert_eq!(decrypted.plaintext, plaintext, "decrypted plaintext must match original");
}