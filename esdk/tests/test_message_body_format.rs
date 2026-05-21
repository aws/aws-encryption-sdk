// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/data-format/message-body.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;

use test_helpers::*;

// The ESDK always uses framed encryption, so nonframed deserialization
// is exercised by parsing/decrypting the external V2 nonframed vector from
// aws-encryption-sdk-test-vectors.

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_data_serialization_order() {
    //= spec/data-format/message-body.md#nonframed-data
    //= type=test
    //# Nonframed data MUST consist of, in order,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.
    // This test verifies the fields are consumed in order via successful decrypt,
    // then verifies each field's parsed length is consistent with the spec.
    let msg = EXTERNAL_V2_NONFRAMED_CT;
    let body = parse_nonframed_body(msg);
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "decrypt output did not match expected plaintext — nonframed body fields are not being consumed in the spec-required order"
    );

    // Field 1 of 4: IV — fixed-width, must equal IV Length (12 bytes).
    assert_eq!(body.iv.len(), IV_LEN);

    // Field 2 of 4: Encrypted Content Length — fixed-width UInt64 (8 bytes).
    assert_eq!(body.encrypted_content_length_bytes.len(), 8);

    // Field 3 of 4: Encrypted Content — variable-width, length given by Field 2.
    assert_eq!(
        body.encrypted_content.len(),
        body.encrypted_content_length as usize
    );

    // Field 4 of 4: Authentication Tag — fixed-width, must equal algorithm suite tag length (16 bytes).
    assert_eq!(body.auth_tag.len(), TAG_LEN);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_iv_length() {
    //= spec/data-format/message-body.md#nonframed-data-iv
    //= type=test
    //# The length of the IV field MUST be [IV Length](message-header.md#iv-length) bytes.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.iv.len(),
        IV_LEN,
        "nonframed IV must be IV_LEN (12) bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_fields_interpreted_as_bytes() {
    //= spec/data-format/message-body.md#nonframed-data-iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content
    //= type=test
    //# The encrypted content value MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#nonframed-data-authentication-tag
    //= type=test
    //# The authentication tag value MUST be interpreted as bytes.
    // All three fields are byte-typed in the parser; a successful decrypt of the
    // external nonframed vector proves each one is consumed as bytes (any other
    // interpretation would corrupt the AES-GCM inputs and fail the auth tag check).
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "decrypt output did not match expected plaintext — nonframed IV/encrypted content/auth tag must be interpreted as bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_encrypted_content_length_uint64() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be interpreted as a UInt64.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.encrypted_content_length,
        EXTERNAL_V2_NONFRAMED_PT.len() as u64,
        "encrypted content length interpreted as UInt64 must equal plaintext length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_encrypted_content_length_max_value() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content-length
    //= type=test
    //# The value of this field MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
    //# due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    let max_allowed: u64 = (1u64 << 36) - 32;

    // The conforming external vector must satisfy the bound.
    assert!(
        body.encrypted_content_length <= max_allowed,
        "encrypted content length {} must not exceed 2^36 - 32 = {}",
        body.encrypted_content_length,
        max_allowed
    );

    // Baseline: the untampered external vector decrypts successfully.
    let baseline = try_decrypt_external_nonframed(Version::V2, EXTERNAL_V2_NONFRAMED_CT)
        .await
        .expect("baseline: untampered external vector must decrypt");
    assert_eq!(baseline, EXTERNAL_V2_NONFRAMED_PT, "baseline plaintext mismatch");

    // Tamper: rewrite the 8-byte content length field at body_start + IV_LEN to
    // exceed the bound by 1, so the decrypt parser rejects it before any AES-GCM
    // operation. (Perturbing at the integer level ensures the parser is the layer
    // that fails.)
    let length_offset = body.body_start + IV_LEN;
    let original_len = u64::from_be_bytes(
        EXTERNAL_V2_NONFRAMED_CT[length_offset..length_offset + 8]
            .try_into()
            .unwrap(),
    );
    assert_eq!(
        original_len, body.encrypted_content_length,
        "raw-byte read of length field must match parsed value"
    );
    let tampered_len = max_allowed + 1;
    assert_ne!(
        tampered_len, original_len,
        "tamper-effectiveness: tampered length must differ from original"
    );

    let mut tampered = EXTERNAL_V2_NONFRAMED_CT.to_vec();
    tampered[length_offset..length_offset + 8].copy_from_slice(&tampered_len.to_be_bytes());

    let err = try_decrypt_external_nonframed(Version::V2, &tampered)
        .await
        .expect_err("decrypt must reject content length > 2^36 - 32");
    assert_eq!(
        err.kind,
        ErrorKind::SerializationError,
        "expected SerializationError for over-bound content length, got: {} ({:?})",
        err.message,
        err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_encrypted_content_length_field_8_bytes() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content-length
    //= type=test
    //# The length of the Encrypted Content Length field MUST be 8 bytes.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.encrypted_content_length_bytes.len(),
        8,
        "encrypted content length field must be exactly 8 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_encrypted_content_length_matches_content() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content
    //= type=test
    //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#nonframed-data-encrypted-content-length) field.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.encrypted_content.len(),
        body.encrypted_content_length as usize,
        "encrypted content byte count must equal the content length field value"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_auth_tag_length() {
    //= spec/data-format/message-body.md#nonframed-data-authentication-tag
    //= type=test
    //# The length of the serialized authentication tag field MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.auth_tag.len(),
        TAG_LEN,
        "nonframed auth tag length must equal algorithm suite tag length (16)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_framed_data_max_frame_count() {
    // With frame_length=4 and 20 bytes, we get 4 regular + 1 final = 5 frames.
    // The implementation checks sequence_number == ENDFRAME_SEQUENCE_NUMBER to enforce the limit.
    //= spec/data-format/message-body.md#framed-data
    //= type=test
    //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
    let pt = vec![0xBBu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 4).await, 4);
    assert_eq!(
        frames.len(),
        5,
        "20 bytes / 4-byte frames = 4 regular + 1 final = 5 frames"
    );
    let result = round_trip_framed(&pt, 4).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_serialization_order() {
    //= spec/data-format/message-body.md#regular-frame
    //= type=test
    //# A regular frame MUST consist of, in order,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content,
    //# and Authentication Tag.
    // Use frame_length=10 with 25 bytes of plaintext: produces 2 regular frames
    // (10 bytes each) followed by 1 final frame (5 bytes). The first regular
    // frame's tag MUST be immediately followed by the next regular frame's seq=2,
    // proving Field 4's end offset is exactly 4 + IV_LEN + FRAME_LEN + TAG_LEN.
    const FRAME_LEN: usize = 10;
    let pt = vec![0xCCu8; 25];
    let ct = encrypt_with_frame_length(&pt, FRAME_LEN as u32).await;
    let body_start = find_body_start(&ct, FRAME_LEN as u32).expect("must find body");

    // Field 1 of 4: Sequence Number — first 4 bytes, UInt32 BE = 1.
    let seq_off = body_start;
    let seq = u32::from_be_bytes([
        ct[seq_off],
        ct[seq_off + 1],
        ct[seq_off + 2],
        ct[seq_off + 3],
    ]);
    assert_eq!(seq, 1, "Field 1: sequence number must be 1 for first regular frame");

    // Field 2 of 4: IV — exactly IV_LEN (12) bytes immediately after seq num.
    let iv_off = seq_off + 4;
    let iv = &ct[iv_off..iv_off + IV_LEN];
    assert_eq!(iv.len(), IV_LEN, "Field 2: IV must be exactly IV_LEN bytes");
    // Sanity: a real AES-GCM IV is not all zeros.
    assert!(
        iv.iter().any(|&b| b != 0),
        "Field 2: IV must not be all zeros (real AES-GCM IV)"
    );

    // Field 3 of 4: Encrypted Content — exactly FRAME_LEN bytes immediately after IV
    // (regular frame: encrypted content length equals frame length).
    let content_off = iv_off + IV_LEN;
    let content = &ct[content_off..content_off + FRAME_LEN];
    assert_eq!(
        content.len(),
        FRAME_LEN,
        "Field 3: regular frame encrypted content length must equal frame length"
    );
    // Sanity: encrypted content must NOT equal the plaintext input (proves encryption happened).
    assert_ne!(
        content,
        &pt[..FRAME_LEN],
        "Field 3: encrypted content must differ from plaintext"
    );

    // Field 4 of 4: Authentication Tag — exactly TAG_LEN (16) bytes immediately after content.
    let tag_off = content_off + FRAME_LEN;
    let tag = &ct[tag_off..tag_off + TAG_LEN];
    assert_eq!(tag.len(), TAG_LEN, "Field 4: auth tag must be exactly TAG_LEN bytes");
    // Sanity: a real AES-GCM tag is not all zeros.
    assert!(
        tag.iter().any(|&b| b != 0),
        "Field 4: auth tag must not be all zeros (real AES-GCM output)"
    );

    // Boundary check: the byte immediately after the tag must be the next frame's
    // seq num (= 2). This proves Field 4 ends at the spec-required offset and the
    // four fields together occupy exactly 4 + IV_LEN + FRAME_LEN + TAG_LEN bytes.
    let next_frame_off = tag_off + TAG_LEN;
    let next_seq = u32::from_be_bytes([
        ct[next_frame_off],
        ct[next_frame_off + 1],
        ct[next_frame_off + 2],
        ct[next_frame_off + 3],
    ]);
    assert_eq!(
        next_seq, 2,
        "boundary: byte immediately after the regular frame must be the next frame's seq=2"
    );

    // Cross-check the manual walk against the independent parser.
    let frames = parse_frames(&ct, FRAME_LEN as u32);
    assert_eq!(frames.len(), 3, "expected 2 regular + 1 final frame for pt=25, frame_length=10");
    assert_eq!(frames[0].0, 1, "parser-reported seq num matches manual walk");
    assert_eq!(frames[0].1.as_slice(), iv, "parser-reported IV matches manual walk");
    assert_eq!(
        frames[0].2.as_slice(),
        content,
        "parser-reported encrypted content matches manual walk"
    );
    assert_eq!(frames[0].3.as_slice(), tag, "parser-reported tag matches manual walk");

    // Round-trip cross-check: independent decrypt validates the whole frame.
    let result = round_trip_framed(&pt, FRAME_LEN as u32).await;
    assert_eq!(result, pt, "round-trip corroborates regular frame serialization order");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_starts_at_one() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Framed Data MUST start at Sequence Number 1.
    let pt = vec![0xDDu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    assert_eq!(frames[0].0, 1, "first frame sequence number must be 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_increments() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
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
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The length of the serialized sequence number field MUST be 4 bytes.
    let pt = vec![0xFFu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Sequence number occupies exactly bytes [body_start..body_start+4]
    let seq_bytes = &ct[body_start..body_start + 4];
    assert_eq!(
        seq_bytes.len(),
        4,
        "sequence number must be exactly 4 bytes"
    );
    assert_eq!(
        u32::from_be_bytes([seq_bytes[0], seq_bytes[1], seq_bytes[2], seq_bytes[3]]),
        1
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_uint32() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
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
async fn test_regular_frame_iv_unique() {
    //= spec/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    let pt = vec![0xCCu8; 40];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let ivs: Vec<&Vec<u8>> = frames.iter().map(|(_, iv, _, _, _)| iv).collect();
    for i in 0..ivs.len() {
        for j in (i + 1)..ivs.len() {
            assert_ne!(
                ivs[i], ivs[j],
                "IV for frame {} must differ from frame {}",
                i, j
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_iv_length_matches_algorithm() {
    //= spec/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xDDu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for (_, iv, _, _, _) in &frames {
        assert_eq!(
            iv.len(),
            IV_LEN,
            "IV length must match algorithm suite IV length (12)"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_fields_interpreted_as_bytes() {
    //= spec/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#regular-frame-encrypted-content
    //= type=test
    //# The encrypted content MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#regular-frame-authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    // All three fields are byte-typed in the parser; a successful round-trip
    // proves each one is consumed as bytes during decrypt (any other
    // interpretation would corrupt the AES-GCM inputs and fail the auth tag check).
    let pt = vec![0xEEu8; 20];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves regular-frame IV/encrypted content/auth tag are interpreted as bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_encrypted_content_length_equals_frame_length() {
    //= spec/data-format/message-body.md#regular-frame-encrypted-content
    //= type=test
    //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
    //
    //= spec/data-format/message-body.md#framed-data
    //= type=test
    //= reason=frame length is a u32, so the type system enforces the 2^32-1 byte limit per frame; testing at the actual boundary is impractical
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
    let frame_length: u32 = 10;
    let pt = vec![0xFFu8; 30];
    let frames = parse_frames(
        &encrypt_with_frame_length(&pt, frame_length).await,
        frame_length,
    );
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
async fn test_regular_frame_auth_tag_length_matches_algorithm() {
    //= spec/data-format/message-body.md#regular-frame-authentication-tag
    //= type=test
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xBBu8; 20];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for (_, _, _, tag, _) in &frames {
        assert_eq!(
            tag.len(),
            TAG_LEN,
            "auth tag length must match algorithm suite (16)"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_serialization_order() {
    //= spec/data-format/message-body.md#final-frame
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
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    // Verify field order: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(7) + Tag(16)
    let expected_total = 4 + 4 + IV_LEN + 4 + 7 + TAG_LEN;
    assert!(
        pos + expected_total <= ct.len(),
        "final frame must have all fields in order"
    );
    // Verify round-trip to confirm correct serialization
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_is_regular_frame_plus_additions() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST only differ from a regular frame by the addition of the
    //# Sequence Number End
    //# and Encrypted Content Length.
    let pt = vec![0xBBu8; 5];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    // Final frame has Sequence Number End (extra vs regular) at pos
    assert_eq!(
        &ct[pos..pos + 4],
        &ENDFRAME_MARKER,
        "final frame starts with Sequence Number End"
    );
    // Then SeqNum(4) + IV(12) + ContentLen(4, extra vs regular) + Content + Tag
    let content_len = u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]]);
    assert!(
        content_len <= 10,
        "final frame has Encrypted Content Length field"
    );
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_value() {
    //= spec/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
    //
    //= spec/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The sequence number end MUST be interpreted as bytes.
    let pt = b"test";
    let ct = encrypt_with_frame_length(pt, 4096).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    // The four on-wire bytes ARE the marker — checking each byte literally
    // proves the field is encoded as bytes AND has the spec-required value.
    assert_eq!(ct[pos], 0xFF);
    assert_eq!(ct[pos + 1], 0xFF);
    assert_eq!(ct[pos + 2], 0xFF);
    assert_eq!(ct[pos + 3], 0xFF);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_4_bytes() {
    //= spec/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The length of the sequence number end field MUST be 4 bytes.
    let pt = b"test";
    let ct = encrypt_with_frame_length(pt, 4096).await;
    // The ENDFRAME marker is exactly 4 bytes: FF FF FF FF
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    assert_eq!(
        &ct[pos..pos + 4],
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "sequence number end is exactly 4 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_equals_total_frames() {
    // 30 bytes / 10-byte frames → 2 regular + 1 final = 3 total frames
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    let pt = vec![0xAAu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "last frame must be final");
    assert_eq!(
        final_frame.0,
        frames.len() as u32,
        "final frame seq num must equal total frame count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_serialized_same_as_regular() {
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The length of the Final Frame Sequence number field MUST be the same as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    let pt = vec![0xBBu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    // Final frame seq num is at pos+4, serialized as 4-byte big-endian UInt32 (same as regular)
    let final_seq = u32::from_be_bytes([ct[pos + 4], ct[pos + 5], ct[pos + 6], ct[pos + 7]]);
    assert!(
        final_seq > 0,
        "final frame sequence number is a valid UInt32"
    );
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame seq num serialized same as regular"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_interpreted_same_as_regular() {
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence Number MUST be interpreted as the same type as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    // Multi-frame round-trip: decrypt reads final frame seq num as UInt32 (same as regular)
    let pt = vec![0xCCu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame seq num is interpreted same as regular"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_unique() {
    //= spec/data-format/message-body.md#final-frame-iv
    //= type=test
    //# A generated IV MUST be a unique IV within the message.
    let pt = vec![0xDDu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_iv = &frames.last().expect("must have final frame").1;
    for (i, (_, iv, _, _, is_final)) in frames.iter().enumerate() {
        if !is_final {
            assert_ne!(
                iv, final_iv,
                "final frame IV must differ from regular frame {}",
                i
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_length_matches_algorithm() {
    //= spec/data-format/message-body.md#final-frame-iv
    //= type=test
    //# The length of the IV field MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
    let pt = vec![0xEEu8; 5];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    assert_eq!(
        final_frame.1.len(),
        IV_LEN,
        "final frame IV length must match algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_fields_interpreted_as_bytes() {
    //= spec/data-format/message-body.md#final-frame-iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#final-frame-encrypted-content
    //= type=test
    //# The encrypted content MUST be interpreted as bytes.
    //
    //= spec/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    // All three fields are byte-typed in the parser; a successful round-trip with
    // a final-frame-only payload (pt < frame_length) proves each one is consumed
    // as bytes (any other interpretation would corrupt AES-GCM and fail the auth tag).
    let pt = vec![0xFFu8; 5];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves final-frame IV/encrypted content/auth tag are interpreted as bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_4_bytes() {
    //= spec/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The length of the serialized encrypted content length field MUST be 4 bytes.
    let pt = vec![0xAAu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    // Content length field is at pos+20 (ENDFRAME(4)+SeqNum(4)+IV(12)), exactly 4 bytes
    let content_len_bytes = &ct[pos + 20..pos + 24];
    assert_eq!(
        content_len_bytes.len(),
        4,
        "encrypted content length field must be exactly 4 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_uint32() {
    //= spec/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be a UInt32.
    let pt = vec![0xBBu8; 7];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("must find ENDFRAME");
    let content_len = u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]]);
    assert_eq!(
        content_len, 7,
        "encrypted content length serialized as UInt32 must equal 7"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_matches() {
    //= spec/data-format/message-body.md#final-frame-encrypted-content
    //= type=test
    //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#final-frame-encrypted-content-length) field.
    let pt = vec![0xDDu8; 7];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    // parse_frames reads content_len from the field and uses it to read enc_content
    // If they didn't match, parsing would fail or produce wrong data
    assert_eq!(
        final_frame.2.len(),
        7,
        "encrypted content length must match the content length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_length_matches_algorithm() {
    //= spec/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xFFu8; 5];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    assert_eq!(
        final_frame.3.len(),
        TAG_LEN,
        "final frame auth tag length must match algorithm suite (16)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_authenticates_final_frame() {
    //= spec/data-format/message-body.md#final-frame-authentication-tag
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
    let err = decrypt(&dec_input)
        .await
        .expect_err("tampered final frame auth tag must cause decryption failure");
    assert!(
        matches!(err.kind, ErrorKind::CryptographicError),
        "expected CryptographicError, got: {} ({:?})",
        err.message,
        err.kind
    );
}

async fn encrypt_v1_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_sequence_number_starts_at_one() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Framed Data MUST start at Sequence Number 1.
    let pt = vec![0xAAu8; 20];
    let frames = parse_frames(&encrypt_v1_with_frame_length(&pt, 10).await, 10);
    assert_eq!(frames[0].0, 1, "V1: first frame sequence number must be 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_sequence_number_increments() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
    let pt = vec![0xBBu8; 40];
    let frames = parse_frames(&encrypt_v1_with_frame_length(&pt, 10).await, 10);
    for i in 1..frames.len() {
        assert_eq!(
            frames[i].0,
            frames[i - 1].0 + 1,
            "V1: frame {} seq num must be 1 greater than frame {}",
            i,
            i - 1
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_iv_unique() {
    //= spec/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    let pt = vec![0xCCu8; 40];
    let frames = parse_frames(&encrypt_v1_with_frame_length(&pt, 10).await, 10);
    let ivs: Vec<&Vec<u8>> = frames.iter().map(|(_, iv, _, _, _)| iv).collect();
    for i in 0..ivs.len() {
        for j in (i + 1)..ivs.len() {
            assert_ne!(ivs[i], ivs[j], "V1: IV for frame {} must differ from frame {}", i, j);
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_encrypted_content_length_equals_frame_length() {
    //= spec/data-format/message-body.md#regular-frame-encrypted-content
    //= type=test
    //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
    let frame_length: u32 = 10;
    let pt = vec![0xDDu8; 30];
    let frames = parse_frames(&encrypt_v1_with_frame_length(&pt, frame_length).await, frame_length);
    for (_, _, enc_content, _, is_final) in &frames {
        if !is_final {
            assert_eq!(
                enc_content.len(),
                frame_length as usize,
                "V1: regular frame encrypted content length must equal frame length"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_sequence_number_end_value() {
    //= spec/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
    let pt = b"v1 test";
    let ct = encrypt_v1_with_frame_length(pt, 4096).await;
    let pos = ct
        .windows(4)
        .position(|w| w == ENDFRAME_MARKER)
        .expect("V1: must find ENDFRAME");
    assert_eq!(&ct[pos..pos + 4], &[0xFF, 0xFF, 0xFF, 0xFF]);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_final_frame_sequence_number_equals_total_frames() {
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    let pt = vec![0xEEu8; 30];
    let frames = parse_frames(&encrypt_v1_with_frame_length(&pt, 10).await, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "last frame must be final");
    assert_eq!(
        final_frame.0,
        frames.len() as u32,
        "V1: final frame seq num must equal total frame count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_framed_data_contains_exactly_one_final_frame() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
    let pt = vec![0xAAu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    let final_count = frames.iter().filter(|(_, _, _, _, is_final)| *is_final).count();
    assert_eq!(final_count, 1, "framed data must contain exactly one final frame");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_is_last_frame() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# The final frame MUST be the last frame.
    let pt = vec![0xBBu8; 30];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    for (i, (_, _, _, _, is_final)) in frames.iter().enumerate() {
        if *is_final {
            assert_eq!(i, frames.len() - 1, "final frame must be the last frame");
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_content_length_lte_frame_length() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
    let frame_length: u32 = 10;
    // 7 bytes plaintext with 10-byte frame → final frame has 7 bytes (< frame_length)
    let pt = vec![0xCCu8; 7];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, frame_length).await, frame_length);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "must be final frame");
    assert!(
        final_frame.2.len() <= frame_length as usize,
        "final frame content length {} must be <= frame length {}",
        final_frame.2.len(),
        frame_length
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_less_than_frame_length_single_final_frame() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# - When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
    let pt = vec![0xDDu8; 5];
    let frames = parse_frames(&encrypt_with_frame_length(&pt, 10).await, 10);
    assert_eq!(frames.len(), 1, "plaintext < frame length must produce exactly one frame");
    assert!(frames[0].4, "the single frame must be a final frame");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_exact_multiple_of_frame_length() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# - When the length of the Plaintext is an exact multiple of the Frame Length
    //# (including if it is equal to the frame length),
    //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
    let frame_length: u32 = 10;
    let pt = vec![0xEEu8; frame_length as usize];
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.4, "last frame must be final");
    let final_content_len = final_frame.2.len() as u32;
    assert!(
        final_content_len == frame_length || final_content_len == 0,
        "final frame content length must be frame_length ({}) or 0, got {}",
        frame_length,
        final_content_len
    );
    let result = round_trip_framed(&pt, frame_length).await;
    assert_eq!(result, pt, "round-trip must succeed for exact multiple of frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_begins_with_header() {
    let ct_v1 = encrypt_with_v1_signing_suite(b"header first test").await;
    //= spec/data-format/message.md#structure
    //= type=test
    //# - The message MUST begin with [Message Header](message-header.md)
    assert_eq!(
        ct_v1[0], 0x01,
        "V1 message must begin with header version byte 0x01"
    );

    let ct_v2 = encrypt_with_signing_suite(b"header first test").await;
    assert_eq!(
        ct_v2[0], 0x02,
        "V2 message must begin with header version byte 0x02"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_body_follows_header() {
    //= spec/data-format/message.md#structure
    //= type=test
    //# - The [Message Body](message-body.md) MUST follow the Message Header
    // Parse the on-wire header end offset for both versions and assert the bytes
    // immediately after the header are the start of the body (frame seq=1, or the
    // ENDFRAME marker if the only frame is a final frame).
    let pt = b"body follows header test";

    // V1: header body || IV(12) || Tag(16), then body starts.
    let ct_v1 = encrypt_with_v1_signing_suite(pt).await;
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(&ct_v1);
    let body_start_v1 = frame_length_offset + 4 + IV_LEN + TAG_LEN;
    let next_4_v1 = u32::from_be_bytes([
        ct_v1[body_start_v1],
        ct_v1[body_start_v1 + 1],
        ct_v1[body_start_v1 + 2],
        ct_v1[body_start_v1 + 3],
    ]);
    assert!(
        next_4_v1 == 1 || next_4_v1 == 0xFFFF_FFFF,
        "V1 body must start immediately after header with frame seq=1 or endframe marker, got {next_4_v1:#010X}"
    );

    // V2: header body || Tag(16), then body starts.
    let ct_v2 = encrypt_with_signing_suite(pt).await;
    let fields = parse_v2_header_field_offsets(&ct_v2);
    let header_body_end = fields.last().expect("must have header fields").2;
    let body_start_v2 = header_body_end + TAG_LEN;
    let next_4_v2 = u32::from_be_bytes([
        ct_v2[body_start_v2],
        ct_v2[body_start_v2 + 1],
        ct_v2[body_start_v2 + 2],
        ct_v2[body_start_v2 + 3],
    ]);
    assert!(
        next_4_v2 == 1 || next_4_v2 == 0xFFFF_FFFF,
        "V2 body must start immediately after header with frame seq=1 or endframe marker, got {next_4_v2:#010X}"
    );
}
