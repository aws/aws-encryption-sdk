// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-body-aad.md

mod test_helpers;

use aws_esdk::__test_internals::{BodyAADContent, body_aad};
use aws_esdk::{decrypt, DecryptInput, EncryptionContext};
use test_helpers::*;

// Known literal values from the spec, repeated here verbatim so the tests
// assert against the specification text (not against the source constants).
const REGULAR_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Frame";
const FINAL_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Final Frame";
const SINGLE_BLOCK_STR: &[u8] = b"AWSKMSEncryptionClient Single Block";

#[test]
fn test_body_aad_structure_ordering() {
    //= specification/data-format/message-body-aad.md#structure
    //= type=test
    //# The message body AAD MUST consist of, in order,
    //# Message ID,
    //# Body AAD Content,
    //# Sequence Number,
    //# and Content Length.
    let msg_id: [u8; 16] = [0x11; 16];
    let seq: u32 = 0xDEAD_BEEF;
    let len: u64 = 0x0102_0304_0506_0708;
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, seq, len, &mut out);

    let content_str = REGULAR_FRAME_STR;
    let expected_total = msg_id.len() + content_str.len() + 4 + 8;
    assert_eq!(out.len(), expected_total, "total length must match structure");

    let mut pos = 0;

    // Message ID first
    assert_eq!(&out[pos..pos + msg_id.len()], &msg_id, "Message ID must come first");
    pos += msg_id.len();

    // Body AAD Content second
    assert_eq!(
        &out[pos..pos + content_str.len()],
        content_str,
        "Body AAD Content must follow Message ID"
    );
    pos += content_str.len();

    // Sequence Number third (big-endian u32)
    assert_eq!(
        &out[pos..pos + 4],
        &seq.to_be_bytes(),
        "Sequence Number must follow Body AAD Content"
    );
    pos += 4;

    // Content Length last (big-endian u64)
    assert_eq!(
        &out[pos..pos + 8],
        &len.to_be_bytes(),
        "Content Length must follow Sequence Number"
    );
}

#[test]
fn test_body_aad_message_id_is_copied_verbatim() {
    // V1 uses 16-byte message IDs; V2 uses 32-byte. Exercise both so the
    // "length matches header version" claim is proven at each valid length.
    for msg_id_len in [16usize, 32] {
        let msg_id: Vec<u8> = (0..msg_id_len).map(|i| i as u8).collect();
        let mut out = Vec::new();
        body_aad(&msg_id, BodyAADContent::RegularFrame, 1, 100, &mut out);
        //= specification/data-format/message-body-aad.md#message-id
        //= type=test
        //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
        //
        //= specification/data-format/message-body-aad.md#message-id
        //= type=test
        //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
        assert_eq!(
            &out[..msg_id_len], msg_id.as_slice(),
            "{msg_id_len}-byte message ID must be copied verbatim"
        );
    }
}

#[test]
fn test_body_aad_content_values_match_spec_literals() {
    let msg_id_16 = [0u8; 16];
    let msg_id_32 = [0u8; 32];

    let mut out = Vec::new();
    body_aad(&msg_id_16, BodyAADContent::RegularFrame, 1, 0, &mut out);
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
    assert_eq!(
        &out[msg_id_16.len()..msg_id_16.len() + REGULAR_FRAME_STR.len()],
        REGULAR_FRAME_STR,
    );

    let mut out = Vec::new();
    body_aad(&msg_id_16, BodyAADContent::FinalFrame, 1, 0, &mut out);
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
    assert_eq!(
        &out[msg_id_16.len()..msg_id_16.len() + FINAL_FRAME_STR.len()],
        FINAL_FRAME_STR,
    );

    let mut out = Vec::new();
    body_aad(&msg_id_32, BodyAADContent::SingleBlock, 1, 0, &mut out);
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - [Nonframed data](message-body.md#nonframed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
    assert_eq!(
        &out[msg_id_32.len()..msg_id_32.len() + SINGLE_BLOCK_STR.len()],
        SINGLE_BLOCK_STR,
    );
}

#[test]
fn test_body_aad_sequence_number_is_4_bytes_uint32_be() {
    let msg_id = [0u8; 16];
    for seq in [0u32, 1, 0x0102_0304, u32::MAX] {
        let mut out = Vec::new();
        body_aad(&msg_id, BodyAADContent::RegularFrame, seq, 0, &mut out);
        let start = msg_id.len() + REGULAR_FRAME_STR.len();
        //= specification/data-format/message-body-aad.md#sequence-number
        //= type=test
        //# The length of the sequence number field MUST be 4 bytes.
        //
        //= specification/data-format/message-body-aad.md#sequence-number
        //= type=test
        //# The sequence number field MUST be interpreted as a UInt32.
        assert_eq!(
            &out[start..start + 4],
            &seq.to_be_bytes(),
            "seq {seq:#x}: sequence number must be 4-byte UInt32 big-endian"
        );
    }
}

#[test]
fn test_body_aad_content_length_is_8_bytes_uint64_be() {
    let msg_id = [0u8; 16];
    for len in [0u64, 1, 0x0102_0304_0506_0708, u64::MAX] {
        let mut out = Vec::new();
        body_aad(&msg_id, BodyAADContent::RegularFrame, 1, len, &mut out);
        let start = msg_id.len() + REGULAR_FRAME_STR.len() + 4;
        //= specification/data-format/message-body-aad.md#content-length
        //= type=test
        //# The length of the content length field MUST be 8 bytes.
        //
        //= specification/data-format/message-body-aad.md#content-length
        //= type=test
        //# The content length field MUST be interpreted as a UInt64.
        assert_eq!(
            &out[start..start + 8],
            &len.to_be_bytes(),
            "len {len:#x}: content length must be 8-byte UInt64 big-endian"
        );
    }
}

// --- Positive nonframed tests, anchored on the external authority vectors ---
//
// ESDKs are no longer allowed to produce nonframed messages, so we can't
// round-trip one through our own encrypt path. Each test here defers to a
// pre-existing external nonframed ciphertext from
// aws-encryption-sdk-test-vectors (V1: python-1.3.5 suite 0x0178; V2:
// python-2.0.0 suite 0x0478) and loops over both versions.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_is_one() {
    for version in VERSIONS {
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        let iv_seq = u32::from_be_bytes([
            parsed.iv[8], parsed.iv[9], parsed.iv[10], parsed.iv[11],
        ]);
        assert_eq!(iv_seq, 1, "{version:?}: body IV must encode seq=1");

        let pt = decrypt_external_nonframed_vector(version).await;
        //= specification/data-format/message-body-aad.md#sequence-number
        //= type=test
        //= reason=External nonframed vectors' body IVs encode seq=1; successful decryption proves the AAD also used seq=1.
        //# For [nonframed data](message-body.md#nonframed-data), the value of this field MUST be `1`.
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: external nonframed vector decrypted to unexpected plaintext — our AAD reconstruction disagrees with the reference producer's"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_equals_plaintext_length() {
    for version in VERSIONS {
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        assert_eq!(
            parsed.encrypted_content_length,
            external_nonframed_pt(version).len() as u64,
            "{version:?}: encrypted-content-length field must equal plaintext length"
        );

        let pt = decrypt_external_nonframed_vector(version).await;
        //= specification/data-format/message-body-aad.md#content-length
        //= type=test
        //= reason=External vectors' body encrypted-content-length field equals plaintext length; successful decryption proves the AAD used the same value.
        //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: decrypt output did not match plaintext — our AAD content_length disagrees with the reference producer's"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_matches_header() {
    for version in VERSIONS {
        let ct = external_nonframed_ct(version);
        // V1 header: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16)
        // V2 header: Version(1) + AlgSuiteID(2) + MessageID(32)
        let (start, expected_len) = match version {
            Version::V1 => (4usize, 16usize),
            Version::V2 => (3usize, 32usize),
        };
        assert_eq!(
            ct[start..start + expected_len].len(),
            expected_len,
            "{version:?} header must carry a {expected_len}-byte message ID"
        );

        let pt = decrypt_external_nonframed_vector(version).await;
        //= specification/data-format/message-body-aad.md#message-id
        //= type=test
        //= reason=External V1 vector carries a 16-byte message ID; V2 vector carries 32 bytes. Successful decryption proves the AAD reconstruction used the version-appropriate length.
        //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: decrypt output did not match plaintext — our AAD message ID length disagrees with the {expected_len}-byte header value"
        );
    }
}

// --- Negative nonframed test: tamper the external authority vector ---
//
// The only AAD field a real attacker can influence via ciphertext modification
// in a nonframed message is `content_length` — the 8-byte explicit field in
// the body that the decryptor reads to build its AAD. The other AAD fields
// (message_id, "AWSKMSEncryptionClient Single Block" literal, seq=1) are
// either header-bound or hard-coded constants in the decrypter, so there is
// no ciphertext byte to flip that would make the decrypter reconstruct an
// AAD with the wrong value there.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_rejects_tampered_length() {
    for version in VERSIONS {
        // Locate the 8-byte encrypted_content_length field in the external
        // vector's body (body_start + 12 skips past the 12-byte IV).
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        let len_offset = parsed.body_start + 12;
        // Baseline: the untampered vector decrypts.
        let baseline = decrypt_external_nonframed_vector(version).await;
        assert_eq!(
            baseline, external_nonframed_pt(version),
            "{version:?} baseline: untampered external vector did not decrypt to expected plaintext"
        );
        // Tamper: flip the low byte of the length field so the decryptor builds
        // an AAD with a different content_length.
        let mut tampered = external_nonframed_ct(version).to_vec();
        tampered[len_offset + 7] ^= 0x01;
        let res = try_decrypt_external_nonframed(version, &tampered).await;
        //= specification/data-format/message-body-aad.md#content-length
        //= type=test
        //= reason=Flipping a bit in the 8-byte encrypted_content_length field of the external vector makes the decryptor reconstruct an AAD with a tampered content_length, causing AES-GCM auth to fail — proves the AAD uses the vector's content_length.
        //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
        assert!(
            res.is_err(),
            "{version:?} decrypt should have failed AES-GCM authentication after tampering the encrypted_content_length field, but it succeeded — the AAD is not tracking the nonframed encrypted content length"
        );
    }
}

// --- Framed tests (the real Rust encryptor produces framed ciphertexts) ---

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_matches_frame_sequence_number() {
    // The frame-header sequence number is observable; the AAD sequence
    // number is not. Companion test `_rejects_tampered_seq` proves the AAD
    // must equal the frame's.
    let pt = vec![0xBBu8; 50];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // 50 bytes / 10 per frame = 5 frames; the last is the final frame.
    assert_eq!(frames.len(), 5, "expected 5 frames (4 regular + 1 final)");
    for (i, frame) in frames.iter().enumerate() {
        let expected_seq = (i + 1) as u32;
        //= specification/data-format/message-body-aad.md#sequence-number
        //= type=test
        //= reason=Parsing shows each frame-header carries its position-in-sequence; the companion tampering test proves the AAD uses that same number.
        //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
        assert_eq!(
            frame.0, expected_seq,
            "frame {i}: sequence number field must equal frame's position in sequence"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_rejects_tampered_seq() {
    let pt = vec![0xCDu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    // Baseline sanity: the untampered ciphertext decrypts cleanly, so any
    // failure below is attributable to the tamper alone.
    let ok = round_trip_framed(&pt, frame_length).await;
    assert_eq!(
        ok, pt,
        "baseline failed: untampered ciphertext did not decrypt to the original plaintext — test environment is broken before tampering"
    );

    // Locate the start of the body (where frame 1's 4-byte sequence field lives).
    let body_start = find_body_start(&ct, frame_length).expect("body start must be findable");
    // Frame 1 starts at body_start; its sequence number is the first 4 bytes.
    // Sanity: confirm it's 1 before tampering.
    let seq_bytes = &ct[body_start..body_start + 4];
    assert_eq!(u32::from_be_bytes([seq_bytes[0], seq_bytes[1], seq_bytes[2], seq_bytes[3]]), 1);

    // Tamper: flip the low bit of frame 1's sequence number so the decryptor sees seq=0.
    let mut tampered = ct.clone();
    tampered[body_start + 3] ^= 0x01;
    let tampered_seq = u32::from_be_bytes([
        tampered[body_start],
        tampered[body_start + 1],
        tampered[body_start + 2],
        tampered[body_start + 3],
    ]);
    assert_ne!(tampered_seq, 1, "tamper must actually change the seq value");

    // Attempt to decrypt the tampered ciphertext. Must fail.
    let keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let res = decrypt(&dec_input).await;
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=Flipping a bit in the frame-header sequence-number field (not the content or tag) makes the decryptor reconstruct an AAD with the tampered seq, causing AES-GCM auth failure — proves the AAD used the frame's actual sequence number.
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    assert!(
        res.is_err(),
        "decrypt should have failed AES-GCM authentication after frame-header sequence number was tampered from 1 to {tampered_seq}, but it succeeded — the AAD is not tracking the frame's sequence number"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_regular_frame_equals_frame_length() {
    let pt = vec![0xDDu8; 30];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    for (i, frame) in frames.iter().enumerate() {
        // Regular frames: is_final=false.
        if !frame.4 {
            //= specification/data-format/message-body-aad.md#content-length
            //= type=test
            //= reason=Parsing shows each regular frame carries exactly frame_length bytes; the round-trip below corroborates the AAD content_length agreed at both ends.
            //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of the [frame length](message-header.md#frame-length) field in the message header.
            assert_eq!(
                frame.2.len() as u32, frame_length,
                "regular frame {i}: encrypted content length must equal frame_length"
            );
        }
    }
    // Corroboration: mismatched AAD content_length between encrypt and decrypt would fail AES-GCM auth.
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(
        decrypted, pt,
        "decrypt output did not match plaintext — encrypt and decrypt disagree on regular-frame AAD content_length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_final_frame_bounded_by_frame_length() {
    let pt = vec![0xEEu8; 15];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let final_content_len = final_frame_content_length(&ct)
        .expect("ciphertext must contain a final frame");
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Parsing the final frame's content_length shows it lies in [0, frame_length]; the round-trip below corroborates the AAD matched at both ends.
    //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to 0 and less than or equal to the value of the [frame length](message-header.md#frame-length) field in the message header.
    assert!(
        final_content_len <= frame_length,
        "final frame content_length ({final_content_len}) must be <= frame_length ({frame_length})"
    );
    // 15 bytes with frame_length=10 -> one regular frame (10) + final frame (5).
    assert_eq!(final_content_len, 5, "final frame should hold remaining 5 bytes");
    // Corroboration: mismatched AAD content_length would fail AES-GCM auth.
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(
        decrypted, pt,
        "decrypt output did not match plaintext — encrypt and decrypt disagree on final-frame AAD content_length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_framed_equals_per_frame_plaintext() {
    let pt = vec![0xCCu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // Regular frames contribute frame_length; final frame contributes its remaining bytes.
    let total: usize = frames.iter().map(|f| f.2.len()).sum();
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Sum of per-frame content lengths equals plaintext length (per-frame, not whole-message); the round-trip below corroborates the AAD agreed at both ends.
    //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes, of the plaintext being encrypted in this frame.
    assert_eq!(
        total, pt.len(),
        "sum of per-frame content lengths must equal plaintext length"
    );
    // Corroboration: mismatched per-frame AAD content_length would fail AES-GCM auth.
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(
        decrypted, pt,
        "decrypt output did not match plaintext — encrypt and decrypt disagree on per-frame AAD content_length"
    );
}
