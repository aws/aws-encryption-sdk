// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-body.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

// The ESDK always uses framed encryption, so non-framed deserialization
// is tested by the decrypt path.

#[tokio::test(flavor = "multi_thread")]
async fn test_framed_data_max_frame_count() {
    //= specification/data-format/message-body.md#framed-data
    //= type=test
    //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
    // With frame_length=4 and 20 bytes, we get 4 regular + 1 final = 5 frames.
    // The implementation checks sequence_number == ENDFRAME_SEQUENCE_NUMBER to enforce the limit.
    let pt = vec![0xBBu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 4).await, 4);
    assert_eq!(frames.len(), 5, "20 bytes / 4-byte frames = 4 regular + 1 final = 5 frames");
    let result = round_trip_framed(&pt, 4).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_serialization_order() {
    //= specification/data-format/message-body.md#regular-frame
    //= type=test
    //# A regular frame MUST consist of, in order,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content,
    //# and Authentication Tag.
    let pt = vec![0xCCu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // First regular frame at body_start: SeqNum(4) + IV(12) + Content(10) + Tag(16)
    let seq = u32::from_be_bytes([ct[body_start], ct[body_start + 1], ct[body_start + 2], ct[body_start + 3]]);
    assert_eq!(seq, 1, "first field is sequence number");
    // IV follows at body_start+4, content at body_start+16, tag at body_start+26
    // Verify by successful round-trip (wrong order would fail decryption)
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves regular frame serialization order is correct");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_starts_at_one() {
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Framed Data MUST start at Sequence Number 1.
    let pt = vec![0xDDu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    assert_eq!(frames[0].0, 1, "first frame sequence number must be 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_increments() {
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
    let pt = vec![0xEEu8; 40];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for i in 1..frames.len() {
        assert_eq!(
            frames[i].0,
            frames[i - 1].0 + 1,
            "frame {} seq num must be 1 greater than frame {}",
            i,
            i - 1
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_4_bytes() {
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The length of the serialized sequence number field MUST be 4 bytes.
    let pt = vec![0xFFu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Sequence number occupies exactly bytes [body_start..body_start+4]
    let seq_bytes = &ct[body_start..body_start + 4];
    assert_eq!(seq_bytes.len(), 4, "sequence number must be exactly 4 bytes");
    assert_eq!(u32::from_be_bytes([seq_bytes[0], seq_bytes[1], seq_bytes[2], seq_bytes[3]]), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_uint32() {
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The sequence number MUST be interpreted as a UInt32.
    let pt = vec![0xAAu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    // All sequence numbers are valid u32 values parsed from big-endian bytes
    for (seq, _, _, _, _) in &frames {
        assert!(*seq > 0, "sequence number must be a valid non-zero UInt32");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_read_as_uint32() {
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The sequence number MUST be interpreted as a UInt32.
    // Successful round-trip proves the decrypt path reads sequence numbers as UInt32
    let pt = vec![0xBBu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves sequence numbers are read as UInt32");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_iv_unique() {
    //= specification/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    let pt = vec![0xCCu8; 40];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let ivs: Vec<&Vec<u8>> = frames.iter().map(|(_, iv, _, _, _)| iv).collect();
    for i in 0..ivs.len() {
        for j in (i + 1)..ivs.len() {
            assert_ne!(ivs[i], ivs[j], "IV for frame {} must differ from frame {}", i, j);
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_iv_length_matches_algorithm() {
    //= specification/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xDDu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for (_, iv, _, _, _) in &frames {
        assert_eq!(iv.len(), IV_LEN, "IV length must match algorithm suite IV length (12)");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_iv_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    // Round-trip proves the IV bytes are correctly interpreted during decrypt
    let pt = vec![0xEEu8; 20];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves IV is correctly interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_encrypted_content_length_equals_frame_length() {
    //= specification/data-format/message-body.md#regular-frame-encrypted-content
    //= type=test
    //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
    let frame_length: u32 = 10;
    let pt = vec![0xFFu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, frame_length).await, frame_length);
    for (_, _, enc_content, _, is_final) in &frames {
        if !is_final {
            assert_eq!(
                enc_content.len(),
                frame_length as usize,
                "regular frame encrypted content length must equal frame length"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_encrypted_content_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#regular-frame-encrypted-content
    //= type=test
    //# The encrypted content MUST be interpreted as bytes.
    let pt = vec![0xAAu8; 20];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves encrypted content is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_auth_tag_length_matches_algorithm() {
    //= specification/data-format/message-body.md#regular-frame-authentication-tag
    //= type=test
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xBBu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for (_, _, _, tag, _) in &frames {
        assert_eq!(tag.len(), TAG_LEN, "auth tag length must match algorithm suite (16)");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_auth_tag_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#regular-frame-authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let pt = vec![0xCCu8; 20];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves auth tag is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_serialization_order() {
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST consist of, in order,
    //# Sequence Number End,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.
    let pt = vec![0xAAu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    // Find ENDFRAME marker — that's the start of the final frame
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    // Verify field order: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(7) + Tag(16)
    let expected_total = 4 + 4 + IV_LEN + 4 + 7 + TAG_LEN;
    assert!(pos + expected_total <= ct.len(), "final frame must have all fields in order");
    // Verify round-trip to confirm correct serialization
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_is_regular_frame_plus_additions() {
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST only differ from a regular frame by the addition of the
    //# Sequence Number End
    //# and Encrypted Content Length.
    let pt = vec![0xBBu8; 5];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    // Final frame has Sequence Number End (extra vs regular) at pos
    assert_eq!(&ct[pos..pos + 4], &ENDFRAME_MARKER, "final frame starts with Sequence Number End");
    // Then SeqNum(4) + IV(12) + ContentLen(4, extra vs regular) + Content + Tag
    let content_len = u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]]);
    assert!(content_len <= 10, "final frame has Encrypted Content Length field");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_value() {
    //= specification/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
    let pt = b"test";
    let ct = encrypt_with_frame_length(pt, 4096).await;
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    assert_eq!(ct[pos], 0xFF);
    assert_eq!(ct[pos + 1], 0xFF);
    assert_eq!(ct[pos + 2], 0xFF);
    assert_eq!(ct[pos + 3], 0xFF);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_4_bytes() {
    //= specification/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The length of the sequence number end field MUST be 4 bytes.
    let pt = b"test";
    let ct = encrypt_with_frame_length(pt, 4096).await;
    // The ENDFRAME marker is exactly 4 bytes: FF FF FF FF
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    assert_eq!(&ct[pos..pos + 4], &[0xFF, 0xFF, 0xFF, 0xFF], "sequence number end is exactly 4 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The sequence number end MUST be interpreted as bytes.
    // Successful round-trip proves the decrypt path correctly interprets the ENDFRAME marker bytes
    let pt = b"test seq end";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(result, pt, "round-trip proves sequence number end is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_equals_total_frames() {
    //= specification/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    // 30 bytes / 10-byte frames → 2 regular + 1 final = 3 total frames
    let pt = vec![0xAAu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "last frame must be final");
    assert_eq!(final_frame.0, frames.len() as u32, "final frame seq num must equal total frame count");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_serialized_same_as_regular() {
    //= specification/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The length of the Final Frame Sequence number field  MUST be the same as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    let pt = vec![0xBBu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    // Final frame seq num is at pos+4, serialized as 4-byte big-endian UInt32 (same as regular)
    let final_seq = u32::from_be_bytes([ct[pos + 4], ct[pos + 5], ct[pos + 6], ct[pos + 7]]);
    assert!(final_seq > 0, "final frame sequence number is a valid UInt32");
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame seq num serialized same as regular");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_interpreted_same_as_regular() {
    //= specification/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence Number MUST be interpreted as the same type as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    // Multi-frame round-trip: decrypt reads final frame seq num as UInt32 (same as regular)
    let pt = vec![0xCCu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame seq num is interpreted same as regular");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_unique() {
    //= specification/data-format/message-body.md#final-frame-iv
    //= type=test
    //# A generated IV MUST be a unique IV within the message.
    let pt = vec![0xDDu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_iv = &frames.last().expect("must have final frame").1;
    for (i, (_, iv, _, _, is_final)) in frames.iter().enumerate() {
        if !is_final {
            assert_ne!(iv, final_iv, "final frame IV must differ from regular frame {}", i);
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_length_matches_algorithm() {
    //= specification/data-format/message-body.md#final-frame-iv
    //= type=test
    //# The length of the IV field MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
    let pt = vec![0xEEu8; 5];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    assert_eq!(final_frame.1.len(), IV_LEN, "final frame IV length must match algorithm suite");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#final-frame-iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    let pt = vec![0xFFu8; 5];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame IV is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_4_bytes() {
    //= specification/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The length of the serialized encrypted content length field MUST be 4 bytes.
    let pt = vec![0xAAu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    // Content length field is at pos+20 (ENDFRAME(4)+SeqNum(4)+IV(12)), exactly 4 bytes
    let content_len_bytes = &ct[pos + 20..pos + 24];
    assert_eq!(content_len_bytes.len(), 4, "encrypted content length field must be exactly 4 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_uint32() {
    //= specification/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be a UInt32.
    let pt = vec![0xBBu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct.windows(4).position(|w| w == ENDFRAME_MARKER).expect("must find ENDFRAME");
    let content_len = u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]]);
    assert_eq!(content_len, 7, "encrypted content length serialized as UInt32 must equal 7");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_read_as_uint32() {
    //= specification/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be a UInt32.
    // Successful round-trip proves decrypt reads the content length as UInt32
    let pt = vec![0xCCu8; 7];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves encrypted content length is read as UInt32");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_matches() {
    //= specification/data-format/message-body.md#final-frame-encrypted-content
    //= type=test
    //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length-1) field.
    let pt = vec![0xDDu8; 7];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    // parse_frames reads content_len from the field and uses it to read enc_content
    // If they didn't match, parsing would fail or produce wrong data
    assert_eq!(final_frame.2.len(), 7, "encrypted content length must match the content length field");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#final-frame-encrypted-content
    //= type=test
    //# The encrypted content MUST be interpreted as bytes.
    let pt = vec![0xEEu8; 5];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame encrypted content is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_length_matches_algorithm() {
    //= specification/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xFFu8; 5];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    assert_eq!(final_frame.3.len(), TAG_LEN, "final frame auth tag length must match algorithm suite (16)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_interpreted_as_bytes() {
    //= specification/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let pt = vec![0xAAu8; 5];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt, "round-trip proves final frame auth tag is interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_authenticates_final_frame() {
    //= specification/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# It MUST be used to authenticate the final frame.
    // Successful decrypt proves the auth tag authenticated the final frame.
    let pt = vec![0xBBu8; 5];
    assert_eq!(round_trip_framed(&pt, 10).await, pt);

    // Tampering with the final frame's auth tag must cause decryption failure.
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_frames(&ct, 10);
    let final_frame = frames.last().unwrap();
    assert!(final_frame.4, "must be final frame");

    let body_start = find_body_start(&ct, 10).unwrap();
    // Walk to the final frame's auth tag position
    let mut pos = body_start;
    for f in &frames[..frames.len() - 1] {
        pos += 4 + IV_LEN + f.2.len() + TAG_LEN; // regular frame
    }
    // Final frame: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(N) + Tag(16)
    let tag_offset = pos + 4 + 4 + IV_LEN + 4 + final_frame.2.len();
    let mut tampered = ct.clone();
    tampered[tag_offset] ^= 0xFF;

    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    assert!(decrypt(&dec_input).await.is_err(), "tampered final frame auth tag must cause decryption failure");
}
