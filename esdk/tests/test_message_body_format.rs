// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/data-format/message-body.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
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
async fn test_nonframed_encrypted_content_length_uint64() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be interpreted as a UInt64.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    let pt_len_u64 =
        u64::try_from(EXTERNAL_V2_NONFRAMED_PT.len()).expect("plaintext length fits in u64");
    assert_eq!(
        body.encrypted_content_length, pt_len_u64,
        "decoded UInt64 BE content length must equal plaintext length"
    );

    // Per-byte BE check on the on-wire 8-byte field. For pt_len = 10240 = 0x2800:
    //   big-endian:    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x00]
    //   little-endian: [0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // The decoded value alone could match either if pt_len fits in one byte;
    // the byte pattern distinguishes the two.
    let expected = pt_len_u64.to_be_bytes();
    let cl_bytes: &[u8] = &body.encrypted_content_length_bytes;
    assert_eq!(cl_bytes, &expected,
        "on-wire content length bytes must match the BE encoding of the plaintext length");
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
    // Read the field directly from the external vector at the offset where the
    // parser places it (body_start + IV_LEN). The parser then reads the next
    // `encrypted_content_length` bytes for the content. So the boundary between
    // the length field and the content is at body_start + IV_LEN + 8 — proving
    // the field spans exactly 8 bytes on the wire.
    let length_field_offset = body.body_start + IV_LEN;
    let pt_len_u64 =
        u64::try_from(EXTERNAL_V2_NONFRAMED_PT.len()).expect("plaintext length fits in u64");
    let on_wire_bytes = &EXTERNAL_V2_NONFRAMED_CT[length_field_offset..length_field_offset + 8];
    assert_eq!(
        on_wire_bytes,
        &pt_len_u64.to_be_bytes(),
        "on-wire content length field at offset {length_field_offset} must be the 8-byte BE encoding of {pt_len_u64}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_encrypted_content_length_matches_content() {
    //= spec/data-format/message-body.md#nonframed-data-encrypted-content
    //= type=test
    //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#nonframed-data-encrypted-content-length) field.
    // Read the on-wire content-length field bytes directly from the external vector
    // (independent of the parser's slicing), decode as BE UInt64, then compute the
    // span of the on-wire content slice from offsets and assert they match.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    let length_field_offset = body.body_start + IV_LEN;
    let on_wire_length_bytes: [u8; 8] = EXTERNAL_V2_NONFRAMED_CT
        [length_field_offset..length_field_offset + 8]
        .try_into()
        .expect("8 bytes");
    let on_wire_length = u64::from_be_bytes(on_wire_length_bytes);

    // Content slice starts immediately after the 8-byte length field and ends
    // immediately before the auth tag. Compute its span from offsets:
    //   content_start = body_start + IV_LEN + 8
    //   content_end   = total_len - TAG_LEN
    let content_start = length_field_offset + 8;
    let content_end = EXTERNAL_V2_NONFRAMED_CT.len() - TAG_LEN;
    let on_wire_content_len = u64::try_from(content_end - content_start)
        .expect("content length fits in u64");

    assert_eq!(
        on_wire_content_len, on_wire_length,
        "on-wire content slice length must equal the value of the on-wire content-length field"
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
    //= spec/data-format/message-body.md#framed-data
    //= type=test
    //= reason=The 2^32-1 boundary is unreachable in a unit test (would require ~4B frames). The implementation enforces it at body_encrypt.rs by checking sequence_number == ENDFRAME_SEQUENCE_NUMBER (0xFFFFFFFF) before writing each regular frame and returning ValidationError("Too many frames"). This test exercises a small frame count to confirm the multi-frame path works; the upper bound itself is enforced by source-side citation.
    //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
    let pt = vec![0xBBu8; 20];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, 4, &keyring).await;
    let frames = parse_all_frames(&ct, 4);
    assert_eq!(
        frames.len(),
        5,
        "20 bytes / 4-byte frames = 4 regular + 1 final = 5 frames"
    );
    let result = decrypt_with_keyring(&ct, &keyring).await;
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
    const FRAME_LEN: u32 = 10;
    let pt = vec![0xCCu8; 25];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, FRAME_LEN, &keyring).await;
    let frames = parse_all_frames(&ct, FRAME_LEN);
    assert_eq!(frames.len(), 3, "expected 2 regular + 1 final frame");
    let f = &frames[0];
    assert!(!f.is_final, "first frame must be regular");

    // Field 1 of 4: Sequence Number — first 4 bytes, UInt32 BE = 1.
    assert_eq!(f.seq_num, 1, "Field 1: sequence number must be 1 for first regular frame");
    assert_eq!(f.seq_num_bytes, &[0x00, 0x00, 0x00, 0x01], "Field 1: BE encoding of seq=1");

    // Field 2 of 4: IV — exactly IV_LEN (12) bytes immediately after seq num.
    assert_eq!(f.iv.len(), IV_LEN, "Field 2: IV must be exactly IV_LEN bytes");
    assert_eq!(f.iv_offset, f.seq_num_offset + 4, "Field 2 must immediately follow Field 1");
    assert!(f.iv.iter().any(|&b| b != 0),
        "Field 2: IV must not be all zeros (real AES-GCM IV)");

    // Field 3 of 4: Encrypted Content — exactly FRAME_LEN bytes immediately after IV.
    assert_eq!(f.content.len(), FRAME_LEN as usize,
        "Field 3: regular frame encrypted content length must equal frame length");
    assert_eq!(f.content_offset, f.iv_offset + IV_LEN, "Field 3 must immediately follow Field 2");
    assert_ne!(f.content, &pt[..FRAME_LEN as usize],
        "Field 3: encrypted content must differ from plaintext");

    // Field 4 of 4: Authentication Tag — exactly TAG_LEN bytes immediately after content.
    assert_eq!(f.tag.len(), TAG_LEN, "Field 4: auth tag must be exactly TAG_LEN bytes");
    assert_eq!(f.tag_offset, f.content_offset + FRAME_LEN as usize,
        "Field 4 must immediately follow Field 3");
    assert!(f.tag.iter().any(|&b| b != 0),
        "Field 4: auth tag must not be all zeros (real AES-GCM output)");

    // Boundary: end_offset must equal frame_offset + 4 + IV_LEN + FRAME_LEN + TAG_LEN.
    let expected_total = 4 + IV_LEN + FRAME_LEN as usize + TAG_LEN;
    assert_eq!(f.end_offset, f.frame_offset + expected_total,
        "regular frame must occupy exactly {} bytes", expected_total);

    // Boundary: the next frame must begin at frame[0].end_offset with seq=2.
    assert_eq!(frames[1].frame_offset, f.end_offset,
        "next frame must begin immediately after the previous frame's tag");
    assert_eq!(frames[1].seq_num, 2,
        "next regular frame's seq must be 2 (proves Field 4 ends at the spec-required offset)");

    // Round-trip cross-check.
    let result = decrypt_with_keyring(&ct, &keyring).await;
    assert_eq!(result, pt, "round-trip corroborates regular frame serialization order");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_starts_at_one() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Framed Data MUST start at Sequence Number 1.
    let pt = vec![0xDDu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames[0].seq_num, 1, "first frame sequence number must be 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_increments() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
    let pt = vec![0xEEu8; 40];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for i in 1..frames.len() {
        assert_eq!(
            frames[i].seq_num,
            frames[i - 1].seq_num + 1,
            "frame {} seq num must be 1 greater than frame {}",
            i,
            i - 1
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_sequence_number_uint32_4_bytes_be() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The length of the serialized sequence number field MUST be 4 bytes.
    //
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# The sequence number MUST be interpreted as a UInt32.
    let pt = vec![0xFFu8; 20];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let f = &parse_all_frames(&ct, 10)[0];
    assert!(!f.is_final, "first frame must be regular");

    // Per rust-conventions: prove byte-order by asserting individual bytes,
    // not just the decoded value. For seq=1 on the wire:
    //   big-endian:    [0x00, 0x00, 0x00, 0x01]  → decodes to 1
    //   little-endian: [0x01, 0x00, 0x00, 0x00]  → also decodes to 1 via from_le_bytes
    // The decoded value alone can't distinguish the two; the byte pattern can.
    assert_eq!(f.seq_num_bytes[0], 0x00, "seq num byte 0 must be 0x00 for big-endian UInt32 = 1");
    assert_eq!(f.seq_num_bytes[1], 0x00, "seq num byte 1 must be 0x00 for big-endian UInt32 = 1");
    assert_eq!(f.seq_num_bytes[2], 0x00, "seq num byte 2 must be 0x00 for big-endian UInt32 = 1");
    assert_eq!(f.seq_num_bytes[3], 0x01, "seq num byte 3 must be 0x01 for big-endian UInt32 = 1");
    assert_eq!(f.seq_num, 1, "decoded UInt32 BE seq num must equal 1");

    // 4-byte field width: the parser computes iv_offset = seq_num_offset + 4, so the
    // boundary between Field 1 (seq num) and Field 2 (IV) is at seq_num_offset + 4.
    assert_eq!(f.iv_offset - f.seq_num_offset, 4,
        "seq num field must span exactly 4 bytes (Field 1 ends where Field 2 begins)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_iv_unique() {
    //= spec/data-format/message-body.md#regular-frame-iv
    //= type=test
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    let pt = vec![0xCCu8; 40];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for i in 0..frames.len() {
        for j in (i + 1)..frames.len() {
            assert_ne!(
                frames[i].iv, frames[j].iv,
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for f in &frames {
        assert_eq!(
            f.iv.len(),
            IV_LEN,
            "IV length must match algorithm suite IV length (12)"
        );
    }
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
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_all_frames(&ct, frame_length);
    for f in &frames {
        if !f.is_final {
            assert_eq!(
                f.content.len(),
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for f in &frames {
        assert_eq!(
            f.tag.len(),
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
    const FRAME_LEN: u32 = 10;
    const PT_LEN: usize = 7;
    let pt = vec![0xAAu8; PT_LEN];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, FRAME_LEN, &keyring).await;
    let f = parse_final_frame(&ct, FRAME_LEN);

    // Field 1 of 6: Sequence Number End — 4 bytes equal to 0xFF FF FF FF.
    assert_eq!(
        f.endframe_marker_bytes.expect("final frame has marker"),
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "Field 1: sequence number end must be 0xFFFFFFFF"
    );

    // Field 2 of 6: Sequence Number — UInt32 BE; with PT_LEN < FRAME_LEN there is exactly
    // one frame, so seq_num = total frame count = 1.
    assert_eq!(f.seq_num_bytes, &[0x00, 0x00, 0x00, 0x01], "Field 2: seq num bytes BE");
    assert_eq!(f.seq_num, 1, "Field 2: decoded seq num");
    assert_eq!(f.seq_num_offset, f.endframe_marker_offset.unwrap() + 4,
        "Field 2 must immediately follow Field 1");

    // Field 3 of 6: IV — exactly IV_LEN bytes immediately after seq num.
    assert_eq!(f.iv.len(), IV_LEN, "Field 3: IV length");
    assert_eq!(f.iv_offset, f.seq_num_offset + 4, "Field 3 must immediately follow Field 2");
    assert!(f.iv.iter().any(|&b| b != 0), "Field 3: IV must not be all zeros (real AES-GCM IV)");

    // Field 4 of 6: Encrypted Content Length — UInt32 BE = PT_LEN.
    let cl_bytes = f.content_length_bytes.expect("final frame has content length");
    assert_eq!(cl_bytes, &[0x00, 0x00, 0x00, PT_LEN as u8],
        "Field 4: content length bytes BE for PT_LEN={}", PT_LEN);
    assert_eq!(f.content_length, PT_LEN as u32, "Field 4: decoded content length");
    assert_eq!(f.content_length_offset.unwrap(), f.iv_offset + IV_LEN,
        "Field 4 must immediately follow Field 3");

    // Field 5 of 6: Encrypted Content — exactly content_length bytes; must NOT equal plaintext.
    assert_eq!(f.content.len(), PT_LEN, "Field 5: content length matches Field 4 value");
    assert_eq!(f.content_offset, f.content_length_offset.unwrap() + 4,
        "Field 5 must immediately follow Field 4");
    assert_ne!(f.content, &pt[..], "Field 5: encrypted content must differ from plaintext");

    // Field 6 of 6: Authentication Tag — exactly TAG_LEN bytes; must not be all zeros.
    assert_eq!(f.tag.len(), TAG_LEN, "Field 6: tag length");
    assert_eq!(f.tag_offset, f.content_offset + PT_LEN, "Field 6 must immediately follow Field 5");
    assert!(f.tag.iter().any(|&b| b != 0), "Field 6: tag must not be all zeros (real AES-GCM tag)");

    // Boundary: end_offset must equal frame_offset + ENDFRAME(4) + SeqNum(4) + IV + ContentLen(4) + Content + Tag.
    let expected_total = 4 + 4 + IV_LEN + 4 + PT_LEN + TAG_LEN;
    assert_eq!(f.end_offset, f.frame_offset + expected_total,
        "final frame must occupy exactly {} bytes", expected_total);

    // Round-trip cross-check.
    let result = decrypt_with_keyring(&ct, &keyring).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_is_regular_frame_plus_additions() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# A final frame MUST only differ from a regular frame by the addition of the
    //# Sequence Number End
    //# and Encrypted Content Length.
    const FRAME_LEN: u32 = 10;
    let pt = vec![0xBBu8; 5];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, FRAME_LEN, &keyring).await;
    let f = parse_final_frame(&ct, FRAME_LEN);

    // The two ADDITIONS over a regular frame:
    //   1. Sequence Number End at the very start of the final frame.
    assert_eq!(
        f.endframe_marker_bytes.expect("final frame has marker"),
        &ENDFRAME_MARKER,
        "final frame starts with Sequence Number End"
    );
    //   2. Encrypted Content Length between the IV and the encrypted content.
    let cl = f.content_length;
    assert!(cl <= FRAME_LEN, "final frame Encrypted Content Length must be <= frame length");
    assert!(f.content_length_offset.is_some(),
        "final frame must have an Encrypted Content Length offset");

    let result = decrypt_with_keyring(&ct, &keyring).await;
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
    let ct = encrypt_with_frame_length(b"test", 4096).await;
    let f = parse_final_frame(&ct, 4096);
    let bytes = f.endframe_marker_bytes.expect("final frame has marker");
    // Per-byte assertion: the four on-wire bytes ARE the marker — checking each
    // byte literally proves the field is encoded as bytes AND has the spec-required value.
    assert_eq!(bytes[0], 0xFF);
    assert_eq!(bytes[1], 0xFF);
    assert_eq!(bytes[2], 0xFF);
    assert_eq!(bytes[3], 0xFF);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_4_bytes() {
    //= spec/data-format/message-body.md#sequence-number-end
    //= type=test
    //# The length of the sequence number end field MUST be 4 bytes.
    let ct = encrypt_with_frame_length(b"test", 4096).await;
    let f = parse_final_frame(&ct, 4096);
    // The marker spans exactly [endframe_marker_offset .. seq_num_offset], which the
    // parser computes as 4 bytes — corroborated here by the on-wire boundary.
    assert_eq!(
        f.seq_num_offset - f.endframe_marker_offset.unwrap(),
        4,
        "sequence number end must be exactly 4 bytes (Field 1 ends where Field 2 begins)"
    );
    assert_eq!(
        f.endframe_marker_bytes.expect("final frame has marker"),
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "sequence number end bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_equals_total_frames() {
    // 30 bytes / 10-byte frames → 2 regular + 1 final = 3 total frames
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.is_final, "last frame must be final");
    let Ok(total) = u32::try_from(frames.len()) else {
        panic!("frame count exceeds u32::MAX");
    };
    assert_eq!(
        final_frame.seq_num,
        total,
        "final frame seq num must equal total frame count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_sequence_number_serialized_same_as_regular() {
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The length of the Final Frame Sequence number field MUST be the same as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    const FRAME_LEN: u32 = 10;
    let pt = vec![0xBBu8; 20];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, FRAME_LEN, &keyring).await;
    let frames = parse_all_frames(&ct, FRAME_LEN);

    // Find a regular frame and the final frame; their seq num field widths must match.
    let regular = frames.iter().find(|f| !f.is_final).expect("must have a regular frame");
    let final_f = frames.iter().find(|f| f.is_final).expect("must have a final frame");
    assert_eq!(
        regular.seq_num_bytes.len(),
        final_f.seq_num_bytes.len(),
        "final frame seq num field width must equal regular frame seq num field width"
    );
    assert_eq!(regular.seq_num_bytes.len(), 4, "seq num field width must be 4 bytes");

    let result = decrypt_with_keyring(&ct, &keyring).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame seq num serialized same as regular"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_iv_unique() {
    //= spec/data-format/message-body.md#final-frame-iv
    //= type=test
    //# A generated IV MUST be a unique IV within the message.
    let pt = vec![0xDDu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    let final_iv = frames.last().expect("must have final frame").iv;
    for (i, f) in frames.iter().enumerate() {
        if !f.is_final {
            assert_ne!(
                f.iv, final_iv,
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.is_final, "must be final frame");
    assert_eq!(
        final_frame.iv.len(),
        IV_LEN,
        "final frame IV length must match algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_encrypted_content_length_uint32_4_bytes_be() {
    //= spec/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The length of the serialized encrypted content length field MUST be 4 bytes.
    //
    //= spec/data-format/message-body.md#final-frame-encrypted-content-length
    //= type=test
    //# The encrypted content length MUST be a UInt32.
    const FRAME_LEN: u32 = 10;
    const PT_LEN: u8 = 7;
    let pt = vec![0xAAu8; PT_LEN as usize];
    let ct = encrypt_with_frame_length(&pt, FRAME_LEN).await;
    let f = parse_final_frame(&ct, FRAME_LEN);

    let cl_bytes = f.content_length_bytes.expect("final frame has content length");

    // Per rust-conventions: prove byte-order via individual bytes, not via decoded
    // value alone (PT_LEN=7 could decode to 7 from LE [0x07, 0x00, 0x00, 0x00] too).
    assert_eq!(cl_bytes[0], 0x00, "content length byte 0 must be 0x00 for BE UInt32 = 7");
    assert_eq!(cl_bytes[1], 0x00, "content length byte 1 must be 0x00 for BE UInt32 = 7");
    assert_eq!(cl_bytes[2], 0x00, "content length byte 2 must be 0x00 for BE UInt32 = 7");
    assert_eq!(cl_bytes[3], PT_LEN, "content length byte 3 must be PT_LEN for BE UInt32 = 7");

    // 4-byte width: the field spans [content_length_offset .. content_offset], which
    // the parser computes as exactly 4 bytes — corroborated by the on-wire boundary.
    assert_eq!(
        f.content_offset - f.content_length_offset.unwrap(),
        4,
        "content length field must span exactly 4 bytes (Field 4 ends where Field 5 begins)"
    );

    // Decoded value corroboration.
    assert_eq!(f.content_length, PT_LEN as u32, "decoded BE UInt32 content length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_auth_tag_length_matches_algorithm() {
    //= spec/data-format/message-body.md#final-frame-authentication-tag
    //= type=test
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let pt = vec![0xFFu8; 5];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let final_frame = parse_final_frame(&ct, 10);
    assert_eq!(
        final_frame.tag.len(),
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
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, 10, &keyring).await;
    assert_eq!(decrypt_with_keyring(&ct, &keyring).await, pt);

    // Tampering with the final frame's auth tag must cause decryption failure.
    let final_frame = parse_final_frame(&ct, 10);

    let mut tampered = ct.clone();
    tampered[final_frame.tag_offset] ^= 0xFF;

    let dec_input =
        DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
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

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_auth_tag_authenticates_regular_frame() {
    //= spec/data-format/message-body.md#regular-frame-authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    // Tampering with a regular frame's auth tag must cause decryption failure.
    // Use 25 bytes / frame_length=10 so the first frame is a regular frame.
    let pt = vec![0xCCu8; 25];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, 10, &keyring).await;
    let frames = parse_all_frames(&ct, 10);
    let regular = frames.iter().find(|f| !f.is_final).expect("must have a regular frame");

    let mut tampered = ct.clone();
    tampered[regular.tag_offset] ^= 0xFF;

    let dec_input =
        DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input)
        .await
        .expect_err("tampered regular frame auth tag must cause decryption failure");
    assert!(
        matches!(err.kind, ErrorKind::CryptographicError),
        "expected CryptographicError, got: {} ({:?})",
        err.message,
        err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_regular_frame_seq_num_out_of_order_rejected() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
    // Tamper the second regular frame's seq num so it's NOT (previous + 1). Decrypt must
    // reject this with SerializationError ("Sequence number out of order"), per the
    // body_decrypt.rs check `if seq_num != expected_frame { return ser_err(...) }`.
    // Use 25 bytes / frame_length=10 → 2 regular + 1 final, so we have a frame[1] to tamper.
    let pt = vec![0xDDu8; 25];
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, 10, &keyring).await;
    let frames = parse_all_frames(&ct, 10);

    // Baseline: untampered ciphertext decrypts.
    let baseline = decrypt_with_keyring(&ct, &keyring).await;
    assert_eq!(baseline, pt, "baseline plaintext mismatch");

    // Tamper the second frame's seq num field at the integer level (perturb 2 → 3).
    let frame1 = &frames[1];
    assert_eq!(frame1.seq_num, 2, "second frame must have seq=2");
    let original = u32::from_be_bytes([
        ct[frame1.seq_num_offset],
        ct[frame1.seq_num_offset + 1],
        ct[frame1.seq_num_offset + 2],
        ct[frame1.seq_num_offset + 3],
    ]);
    let tampered_seq = original + 1;
    assert_ne!(tampered_seq, original, "tamper-effectiveness");

    let mut tampered = ct.clone();
    tampered[frame1.seq_num_offset..frame1.seq_num_offset + 4]
        .copy_from_slice(&tampered_seq.to_be_bytes());

    let dec_input =
        DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input)
        .await
        .expect_err("out-of-order seq num must be rejected");
    assert_eq!(
        err.kind,
        ErrorKind::SerializationError,
        "expected SerializationError for out-of-order seq num, got: {} ({:?})",
        err.message,
        err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_sequence_number_starts_at_one() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Framed Data MUST start at Sequence Number 1.
    let pt = vec![0xAAu8; 20];
    let ct = encrypt_v1_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames[0].seq_num, 1, "V1: first frame sequence number must be 1");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_regular_frame_sequence_number_increments() {
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= type=test
    //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
    let pt = vec![0xBBu8; 40];
    let ct = encrypt_v1_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for i in 1..frames.len() {
        assert_eq!(
            frames[i].seq_num,
            frames[i - 1].seq_num + 1,
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
    let ct = encrypt_v1_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for i in 0..frames.len() {
        for j in (i + 1)..frames.len() {
            assert_ne!(
                frames[i].iv, frames[j].iv,
                "V1: IV for frame {} must differ from frame {}",
                i, j
            );
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
    let ct = encrypt_v1_with_frame_length(&pt, frame_length).await;
    let frames = parse_all_frames(&ct, frame_length);
    for f in &frames {
        if !f.is_final {
            assert_eq!(
                f.content.len(),
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
    let ct = encrypt_v1_with_frame_length(b"v1 test", 4096).await;
    let f = parse_final_frame(&ct, 4096);
    assert_eq!(
        f.endframe_marker_bytes.expect("V1: final frame has marker"),
        &[0xFF, 0xFF, 0xFF, 0xFF]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_final_frame_sequence_number_equals_total_frames() {
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= type=test
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    let pt = vec![0xEEu8; 30];
    let ct = encrypt_v1_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    let final_frame = frames.last().expect("must have final frame");
    assert!(final_frame.is_final, "last frame must be final");
    let Ok(total) = u32::try_from(frames.len()) else {
        panic!("frame count exceeds u32::MAX");
    };
    assert_eq!(
        final_frame.seq_num,
        total,
        "V1: final frame seq num must equal total frame count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_framed_data_contains_exactly_one_final_frame() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    let final_count = frames.iter().filter(|f| f.is_final).count();
    assert_eq!(final_count, 1, "framed data must contain exactly one final frame");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_final_frame_is_last_frame() {
    //= spec/data-format/message-body.md#final-frame
    //= type=test
    //# The final frame MUST be the last frame.
    let pt = vec![0xBBu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    for (i, f) in frames.iter().enumerate() {
        if f.is_final {
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
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let final_frame = parse_final_frame(&ct, frame_length);
    assert!(
        final_frame.content.len() <= frame_length as usize,
        "final frame content length {} must be <= frame length {}",
        final_frame.content.len(),
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
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames.len(), 1, "plaintext < frame length must produce exactly one frame");
    assert!(frames[0].is_final, "the single frame must be a final frame");
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
    let keyring = test_keyring().await;
    let ct = encrypt_framed_with_keyring(&pt, frame_length, &keyring).await;
    let final_frame = parse_final_frame(&ct, frame_length);
    let Ok(final_content_len) = u32::try_from(final_frame.content.len()) else {
        panic!("final content length exceeds u32::MAX");
    };
    assert!(
        final_content_len == frame_length || final_content_len == 0,
        "final frame content length must be frame_length ({}) or 0, got {}",
        frame_length,
        final_content_len
    );
    let result = decrypt_with_keyring(&ct, &keyring).await;
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
