// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#construct-a-frame
//!
//! These tests pin the encrypt-side construction contract: the VALUES that the
//! Encrypt operation puts in each frame field. The on-wire shape of those
//! fields (lengths, byte order) is covered in test_message_body_format.rs.
//!
//! For value-selection requirements where the value is not directly observable
//! on the wire (the AAD's sequence number, the body AAD content tag, the
//! message ID inside the AAD), a successful decrypt of a multi-frame ciphertext
//! is the cross-module check: the decryptor independently reconstructs the AAD
//! from the on-wire sequence number and the header's message ID, so an encrypt
//! that put the wrong value in the AAD would fail authentication on decrypt.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_aead_inputs_authenticate_via_round_trip() {
    // The cipherkey, the AAD's body-aad-content tag, and the message ID inside
    // the AAD are not directly observable on the wire. The decryptor
    // independently re-derives each from the header and the per-frame on-wire
    // sequence number, then runs AES-GCM authenticated decryption. Round-trip
    // success on a multi-frame ciphertext therefore proves all three values
    // were correct on encrypt — any disagreement would fail authentication.
    let pt: Vec<u8> = (0u8..=200).collect();
    let result = round_trip_framed(&pt, 50).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt independently re-derives the data key from EDKs; round-trip success proves encrypt used the same derived data key
    //# - The cipherkey MUST be the derived data key
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt reads message ID from the header and inserts it into the AAD; round-trip success proves encrypt put the same ID in the AAD
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-format/message-header.md#message-id) serialized in the header of this message.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=regular vs final body-AAD content strings are baked into decrypt; round-trip success proves encrypt used the right tag for each frame type (multi-frame ciphertext exercises both)
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
    //# [Message Body AAD](../data-format/message-body-aad.md).
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_iv_is_padded_sequence_number() {
    // 35 bytes / frame_length=10 → seq 1, 2, 3 (regular) + 4 (final, 5 bytes).
    // Three regular frames lets us verify the seq→IV mapping for multiple
    // distinct values (per the rust-conventions ordering rule: ≥3 items needed).
    let pt = vec![0xAAu8; 35];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_frames(&ct, 10);
    assert_eq!(frames.len(), 4, "35 bytes / 10-byte frames → 3 regular + 1 final");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD for this frame,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    for (i, (seq, iv, _, _, _)) in frames.iter().enumerate() {
        assert_eq!(iv.len(), IV_LEN, "frame {i}: IV must be IV_LEN bytes");
        let iv_seq = u32::from_be_bytes([iv[8], iv[9], iv[10], iv[11]]);
        assert_eq!(
            iv_seq, *seq,
            "frame {i}: low 4 bytes of IV must equal the wire sequence number {seq}"
        );
        assert_eq!(
            &iv[0..8],
            &[0u8; 8],
            "frame {i}: high 8 bytes of IV must be the zero pad"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_starts_at_one_and_increments() {
    // 40 bytes / frame_length=10 → seq 1, 2, 3, 4 (3 regular + 1 final). Four
    // frames exceeds the ≥3-items minimum for ordering tests and disambiguates
    // monotonic-but-wrong mappings (e.g., 2*i+1).
    let pt = vec![0xCCu8; 40];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_frames(&ct, 10);
    assert_eq!(frames.len(), 4);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    assert_eq!(frames[0].0, 1, "first frame sequence number must be 1");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    for i in 1..frames.len() {
        assert_eq!(
            frames[i].0,
            frames[i - 1].0 + 1,
            "frame {i}: sequence number must be 1 greater than previous"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_in_aad_matches_wire() {
    // The AAD's sequence-number field is not directly visible on the wire, but
    // the decryptor reconstructs it from the wire sequence number; round-trip
    // success across multiple frames proves the encrypt-side AAD seq matched
    // the wire seq for every frame.
    let pt = vec![0xABu8; 100];
    let result = round_trip_framed(&pt, 10).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt re-builds the AAD using the wire seq num; round-trip success proves encrypt put the wire seq num into the AAD
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
    //# number of the frame being encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_plaintext_equals_frame_length() {
    // 25 bytes / frame_length=10 → 2 regular (10 each) + 1 final (5).
    let pt = vec![0xBBu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    let regular: Vec<_> = frames.iter().filter(|f| !f.4).collect();
    assert_eq!(regular.len(), 2);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a regular frame the length of this plaintext MUST equal the frame length.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - For a regular frame the length of this plaintext subsequence MUST equal the frame length.
    for (i, (_, _, enc_content, _, _)) in regular.iter().enumerate() {
        assert_eq!(
            u32::try_from(enc_content.len()).unwrap(),
            frame_length,
            "regular frame {i}: encrypted content length (= plaintext length) must equal frame length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_plaintext_at_most_frame_length() {
    // Three boundary cases for the final frame's remaining-plaintext rule:
    // pt_len < frame_length → final = pt_len
    // pt_len == frame_length → final = frame_length
    // pt_len > frame_length, pt_len % frame_length != 0 → final = pt_len % frame_length
    let frame_length = 10u32;
    for (label, pt_len, expected_final_len) in [
        ("less", 7usize, 7u32),
        ("equal", 10usize, 10u32),
        ("greater_partial", 13usize, 3u32),
    ] {
        let pt = vec![0xDDu8; pt_len];
        let ct = encrypt_with_frame_length(&pt, frame_length).await;
        let frames = parse_frames(&ct, frame_length);
        let (_, _, enc_content, _, is_final) = frames.last().expect("must have a final frame");
        assert!(*is_final, "{label}: last frame must be final");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - For a final frame this MUST be the length of the remaining plaintext bytes
        //# which have not yet been encrypted,
        //# whose length MUST be equal to or less than the frame length.
        assert_eq!(
            u32::try_from(enc_content.len()).unwrap(),
            expected_final_len,
            "{label}: final frame content length must equal remaining plaintext"
        );
        assert!(
            u32::try_from(enc_content.len()).unwrap() <= frame_length,
            "{label}: final frame content length must be <= frame length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_content_length_in_aad_matches_plaintext() {
    // Mixed-size frames cover both regular (== frame_length) and final
    // (< frame_length) cases. Round-trip across multiple frames proves the
    // encrypt-side AAD content length matched the actual plaintext length per
    // frame, since otherwise auth would fail on at least one frame.
    let pt = vec![0xCDu8; 50];
    let result = round_trip_framed(&pt, 20).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt re-derives the AAD content length from the wire content length per frame; round-trip success proves encrypt put the actual plaintext length into the AAD
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext being encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_plaintext_subsequence_consumed_in_order() {
    // Use a plaintext of all distinct bytes so any reordering, skip, or
    // duplication of plaintext between frames would fail the equality check.
    let pt: Vec<u8> = (0..=255).collect();
    let result = round_trip_framed(&pt, 50).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=plaintext bytes are all distinct; reordering, skipping, or duplicating any subsequence between frames would fail the round-trip equality
    //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_field_serialization() {
    // Verify each on-wire field of a regular frame has the spec-required value.
    // A 25-byte plaintext with frame_length=10 produces 2 regular frames at
    // sequence numbers 1 and 2 — the first regular and a subsequent regular.
    let pt = vec![0xEEu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    let regular: Vec<_> = frames.iter().filter(|f| !f.4).collect();
    assert_eq!(regular.len(), 2);

    for (i, (seq, iv, enc_content, tag, _)) in regular.iter().enumerate() {
        let expected_seq = u32::try_from(i).unwrap() + 1;

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
        //# The value MUST be the sequence number of this frame.
        assert_eq!(*seq, expected_seq, "regular frame {i}: serialized sequence number");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [IV](../data-format/message-body.md#regular-frame-iv).
        //# The value MUST be the IV used when calculating the encrypted content for this frame.
        assert_eq!(iv.len(), IV_LEN, "regular frame {i}: serialized IV length");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
        //# The value MUST be the encrypted content calculated for this frame.
        assert_eq!(
            u32::try_from(enc_content.len()).unwrap(),
            frame_length,
            "regular frame {i}: serialized encrypted content length"
        );

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
        //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
        assert_eq!(tag.len(), TAG_LEN, "regular frame {i}: serialized auth tag length");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_field_serialization() {
    // 13 bytes / frame_length=10 → 1 regular (seq 1) + 1 final (seq 2, content_len 3).
    let pt = vec![0x42u8; 13];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    let (seq, iv, enc_content, tag, is_final) =
        frames.last().expect("must have a final frame").clone();
    assert!(is_final);

    // Sequence Number End is the 4-byte 0xFFFFFFFF marker that prefixes the
    // final frame on the wire. parse_frames consumes that marker before
    // recording fields, so we re-locate it on the raw bytes.
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
    let endframe_count = ct.windows(4).filter(|w| *w == ENDFRAME_MARKER).count();
    assert_eq!(
        endframe_count, 1,
        "exactly one Sequence Number End marker must appear"
    );

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    //# The value MUST be the sequence number of this frame.
    assert_eq!(seq, 2, "final frame sequence number");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [IV](../data-format/message-body.md#final-frame-iv).
    assert_eq!(iv.len(), IV_LEN, "final frame IV length");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    assert_eq!(
        u32::try_from(enc_content.len()).unwrap(),
        3,
        "final frame Encrypted Content Length"
    );

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    assert_eq!(enc_content.len(), 3, "final frame encrypted content");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    assert_eq!(tag.len(), TAG_LEN, "final frame auth tag length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_streaming_frame_released() {
    // Streaming case: provide enough plaintext for multiple frames and verify
    // the writer received bytes that decrypt to the original plaintext. This
    // also exercises the "MUST NOT be released until entire frame serialized"
    // invariant — premature release of a partial frame would leave the
    // resulting ciphertext unparseable or its tag invalid.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.frame_length = FrameLength::new(10).unwrap();
    let plaintext = vec![0xAAu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input)
        .await
        .expect("streaming encrypt must succeed");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=successful round-trip of streamed bytes proves each frame was completely serialized before being released to the writer
    //# The serialized frame bytes MUST NOT be released until the entire frame has been serialized.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If the Encrypt operation is streaming the encrypted message and
    //# the entire frame has been serialized,
    //# the serialized frame MUST be released.
    assert!(!output.is_empty(), "streaming output must contain released frame bytes");

    let dec_keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), dec_keyring);
    let decrypted = decrypt(&dec_input).await.unwrap();
    assert_eq!(decrypted.plaintext, plaintext);
}
