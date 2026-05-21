// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#construct-a-frame

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_aead_inputs_authenticate_via_round_trip() {
    // Cipherkey, message ID in AAD, and body-AAD-content tag are not on the wire;
    // round-trip across multiple frames is the cross-module check.
    let pt: Vec<u8> = (0u8..=200).collect();
    let result = round_trip_framed(&pt, 50).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt re-derives the data key from EDKs; round-trip success proves agreement
    //# - The cipherkey MUST be the derived data key
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt reads the header message ID and inserts it into the AAD
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-format/message-header.md#message-id) serialized in the header of this message.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=multi-frame ciphertext exercises both regular and final body-AAD content tags
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
    //# [Message Body AAD](../data-format/message-body-aad.md).
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_iv_is_padded_sequence_number() {
    // 35 bytes / frame_length=10 → 3 regular + 1 final (≥3 distinct seq nums).
    let pt = vec![0xAAu8; 35];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_frames(&ct, 10);
    assert_eq!(frames.len(), 4);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD for this frame,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    for (i, (seq, iv, _, _, _)) in frames.iter().enumerate() {
        assert_eq!(iv.len(), IV_LEN, "frame {i}");
        let iv_seq = u32::from_be_bytes([iv[8], iv[9], iv[10], iv[11]]);
        assert_eq!(iv_seq, *seq, "frame {i}: IV low 4 bytes != seq {seq}");
        assert_eq!(&iv[0..8], &[0u8; 8], "frame {i}: IV high 8 bytes != zero pad");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_starts_at_one_and_increments() {
    // 40 bytes / frame_length=10 → 4 frames at seq 1..=4.
    let pt = vec![0xCCu8; 40];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_frames(&ct, 10);
    assert_eq!(frames.len(), 4);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    assert_eq!(frames[0].0, 1);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    for i in 1..frames.len() {
        assert_eq!(frames[i].0, frames[i - 1].0 + 1, "frame {i}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_sequence_number_in_aad_matches_wire() {
    // AAD seq is not on the wire; multi-frame round-trip is the cross-module check.
    let pt = vec![0xABu8; 100];
    let result = round_trip_framed(&pt, 10).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt rebuilds AAD from the wire seq num; round-trip pins encrypt to it
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
    //# number of the frame being encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_plaintext_equals_frame_length() {
    // 25 bytes / frame_length=10 → 2 regular (10) + 1 final (5).
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
            "regular frame {i}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_plaintext_at_most_frame_length() {
    let frame_length = 10u32;
    // (label, pt_len, expected_final_len): less-than, equal, greater-with-partial.
    for (label, pt_len, expected_final_len) in [
        ("less", 7usize, 7u32),
        ("equal", 10usize, 10u32),
        ("greater_partial", 13usize, 3u32),
    ] {
        let pt = vec![0xDDu8; pt_len];
        let ct = encrypt_with_frame_length(&pt, frame_length).await;
        let frames = parse_frames(&ct, frame_length);
        let (_, _, enc_content, _, is_final) = frames.last().expect("final frame");
        assert!(*is_final, "{label}");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - For a final frame this MUST be the length of the remaining plaintext bytes
        //# which have not yet been encrypted,
        //# whose length MUST be equal to or less than the frame length.
        let final_len = u32::try_from(enc_content.len()).unwrap();
        assert_eq!(final_len, expected_final_len, "{label}");
        assert!(final_len <= frame_length, "{label}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_content_length_in_aad_matches_plaintext() {
    // 50 bytes / frame_length=20 mixes regular and final frames.
    let pt = vec![0xCDu8; 50];
    let result = round_trip_framed(&pt, 20).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=decrypt rebuilds AAD from the wire content length; round-trip pins encrypt to it
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext being encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_plaintext_subsequence_consumed_in_order() {
    // All-distinct bytes: any reorder, skip, or duplication breaks equality.
    let pt: Vec<u8> = (0..=255).collect();
    let result = round_trip_framed(&pt, 50).await;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_regular_frame_field_serialization() {
    // 25 bytes / frame_length=10 → 2 regular frames at seq 1 and 2.
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
        assert_eq!(*seq, expected_seq, "frame {i}");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [IV](../data-format/message-body.md#regular-frame-iv).
        //# The value MUST be the IV used when calculating the encrypted content for this frame.
        assert_eq!(iv.len(), IV_LEN, "frame {i}");

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
        //# The value MUST be the encrypted content calculated for this frame.
        assert_eq!(
            u32::try_from(enc_content.len()).unwrap(),
            frame_length,
            "frame {i}"
        );

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= type=test
        //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
        //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
        assert_eq!(tag.len(), TAG_LEN, "frame {i}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_final_frame_field_serialization() {
    // 13 bytes / frame_length=10 → 1 regular (seq 1) + 1 final (seq 2, len 3).
    let pt = vec![0x42u8; 13];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    let (seq, iv, enc_content, tag, is_final) =
        frames.last().expect("final frame").clone();
    assert!(is_final);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
    let endframe_count = ct.windows(4).filter(|w| *w == ENDFRAME_MARKER).count();
    assert_eq!(endframe_count, 1);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    //# The value MUST be the sequence number of this frame.
    assert_eq!(seq, 2);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [IV](../data-format/message-body.md#final-frame-iv).
    assert_eq!(iv.len(), IV_LEN);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    assert_eq!(u32::try_from(enc_content.len()).unwrap(), 3);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    assert_eq!(enc_content.len(), 3);

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    assert_eq!(tag.len(), TAG_LEN);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_streaming_frame_released() {
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.frame_length = FrameLength::new(10).unwrap();
    let plaintext = vec![0xAAu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input)
        .await
        .expect("streaming encrypt");

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=premature release of a partial frame would leave the ciphertext unparseable
    //# The serialized frame bytes MUST NOT be released until the entire frame has been serialized.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# If the Encrypt operation is streaming the encrypted message and
    //# the entire frame has been serialized,
    //# the serialized frame MUST be released.
    assert!(!output.is_empty());

    let dec_keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), dec_keyring);
    let decrypted = decrypt(&dec_input).await.unwrap();
    assert_eq!(decrypted.plaintext, plaintext);
}
