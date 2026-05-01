// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-body-aad.md
//!
//! Direct byte-level tests of `body_aad()` (via `__test_internals`) are
//! followed by end-to-end tests that verify the values body_encrypt /
//! body_decrypt pass to body_aad. See the section headers below for the
//! per-block rationale.

mod test_helpers;

use aws_esdk::__test_internals::{BodyAADContent, body_aad};
use aws_esdk::{decrypt, DecryptInput, EncryptionContext};
use test_helpers::*;

// Known literal values from the spec, repeated here verbatim so the tests
// assert against the specification text (not against the source constants).
const REGULAR_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Frame";
const FINAL_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Final Frame";
const SINGLE_BLOCK_STR: &[u8] = b"AWSKMSEncryptionClient Single Block";

/// Returns the expected body AAD content string for each variant,
/// used for slicing the serialized output.
fn content_str_bytes(bc: BodyAADContent) -> &'static [u8] {
    match bc {
        BodyAADContent::RegularFrame => REGULAR_FRAME_STR,
        BodyAADContent::FinalFrame => FINAL_FRAME_STR,
        BodyAADContent::SingleBlock => SINGLE_BLOCK_STR,
    }
}

// -----------------------------------------------------------------------------
// Direct byte-level tests of body_aad().
// -----------------------------------------------------------------------------

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
fn test_body_aad_message_id_is_copied_verbatim_v1_length() {
    let msg_id: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, 1, 100, &mut out);
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    //
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    assert_eq!(&out[..16], &msg_id, "V1 (16-byte) message ID must be copied verbatim");
}

#[test]
fn test_body_aad_message_id_is_copied_verbatim_v2_length() {
    let mut msg_id = [0u8; 32];
    for (i, b) in msg_id.iter_mut().enumerate() {
        *b = i as u8;
    }
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::SingleBlock, 1, 100, &mut out);
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    //
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    assert_eq!(&out[..32], &msg_id, "V2 (32-byte) message ID must be copied verbatim");
}

#[test]
fn test_body_aad_content_regular_frame_value() {
    let msg_id = [0u8; 16];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::RegularFrame, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + REGULAR_FRAME_STR.len();
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
    assert_eq!(
        &out[start..end],
        REGULAR_FRAME_STR,
        "regular frame content value must be exactly `AWSKMSEncryptionClient Frame`"
    );
}

#[test]
fn test_body_aad_content_final_frame_value() {
    let msg_id = [0u8; 16];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::FinalFrame, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + FINAL_FRAME_STR.len();
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
    assert_eq!(
        &out[start..end],
        FINAL_FRAME_STR,
        "final frame content value must be exactly `AWSKMSEncryptionClient Final Frame`"
    );
}

#[test]
fn test_body_aad_content_single_block_value() {
    let msg_id = [0u8; 32];
    let mut out = Vec::new();
    body_aad(&msg_id, BodyAADContent::SingleBlock, 1, 0, &mut out);
    let start = msg_id.len();
    let end = start + SINGLE_BLOCK_STR.len();
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - [Nonframed data](message-body.md#nonframed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
    assert_eq!(
        &out[start..end],
        SINGLE_BLOCK_STR,
        "nonframed content value must be exactly `AWSKMSEncryptionClient Single Block`"
    );
}

#[test]
fn test_body_aad_content_utf8_encoded() {
    let msg_id = [0u8; 16];
    for bc in [
        BodyAADContent::RegularFrame,
        BodyAADContent::FinalFrame,
        BodyAADContent::SingleBlock,
    ] {
        let mut out = Vec::new();
        body_aad(&msg_id, bc, 1, 0, &mut out);
        let start = msg_id.len();
        let expected = content_str_bytes(bc);
        let end = start + expected.len();
        //= specification/data-format/message-body-aad.md#body-aad-content
        //= type=test
        //# The body AAD content value MUST be encoded as UTF-8 bytes.
        // All three literal strings are ASCII (a strict subset of UTF-8); asserting that
        // the serialized bytes equal the `.as_bytes()` of a Rust `str` proves the
        // encoding is UTF-8 by Rust's type-system guarantees.
        // `expected` is `str::as_bytes()` output — valid UTF-8 by construction.
        std::str::from_utf8(&out[start..end])
            .expect("serialized content bytes must be valid UTF-8");
        assert_eq!(&out[start..end], expected, "{bc:?}: UTF-8 encoded bytes must match literal");
    }
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

// -----------------------------------------------------------------------------
// End-to-end tests: caller contracts on body_aad's inputs.
// -----------------------------------------------------------------------------

// --- Positive nonframed tests, anchored on the external authority vectors ---
//
// ESDKs are no longer allowed to produce nonframed messages, so we can't
// round-trip one through our own encrypt path. Each test here defers to a
// pre-existing external nonframed ciphertext from
// aws-encryption-sdk-test-vectors (V1: python-1.3.5 suite 0x0178; V2:
// python-2.0.0 suite 0x0478) and loops over both versions.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_is_one() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=External nonframed vectors' body IVs encode seq=1; successful decryption proves the AAD also used seq=1.
    //# For [nonframed data](message-body.md#nonframed-data), the value of this field MUST be `1`.
    for version in VERSIONS {
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        let iv_seq = u32::from_be_bytes([
            parsed.iv[8], parsed.iv[9], parsed.iv[10], parsed.iv[11],
        ]);
        assert_eq!(iv_seq, 1, "{version:?}: body IV must encode seq=1");

        let pt = decrypt_external_nonframed_vector(version).await;
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: external nonframed vector must decrypt to the expected plaintext"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_equals_plaintext_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=External vectors' body encrypted-content-length field equals plaintext length; successful decryption proves the AAD used the same value.
    //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
    for version in VERSIONS {
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        assert_eq!(
            parsed.encrypted_content_length,
            external_nonframed_pt(version).len() as u64,
            "{version:?}: encrypted-content-length field must equal plaintext length"
        );

        let pt = decrypt_external_nonframed_vector(version).await;
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: decryption succeeding proves AAD used the same content length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_message_id_length_matches_header() {
    //= specification/data-format/message-body-aad.md#message-id
    //= type=test
    //= reason=External V1 vector carries a 16-byte message ID; V2 vector carries 32 bytes. Successful decryption proves the AAD reconstruction used the version-appropriate length.
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
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
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?} decrypt proves AAD used the {expected_len}-byte header message ID"
        );
    }
}

// --- Negative nonframed tests: tamper AAD fields on self-built messages ---
//
// Since we can't produce a real nonframed ciphertext, we build one with a
// caller-supplied AAD (`build_nonframed_message_with_aad_overrides`). If the
// real decryptor reconstructs the AAD per spec, any override that deviates
// from the spec values makes AES-GCM authentication fail.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_rejects_non_one() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=Nonframed messages built with AAD seq != 1 fail to decrypt — proves the decryptor uses seq=1 exactly.
    //# For [nonframed data](message-body.md#nonframed-data), the value of this field MUST be `1`.
    let pt = b"nonframed seq tamper test";
    for wrong_seq in [0u32, 2, 100, u32::MAX] {
        let msg = build_nonframed_message_with_aad_overrides(
            pt,
            wrong_seq,
            b"AWSKMSEncryptionClient Single Block",
            pt.len() as u64,
        );
        let res = try_decrypt_nonframed(&msg).await;
        assert!(
            res.is_err(),
            "nonframed message with AAD seq={wrong_seq} must fail authentication, but decrypted to {:?}",
            res.ok()
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_nonframed_rejects_wrong_content_string() {
    //= specification/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //= reason=Nonframed messages built with any content string other than the spec literal fail to decrypt — proves the decryptor uses the Single Block literal exactly.
    //# - [Nonframed data](message-body.md#nonframed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
    let pt = b"nonframed content tamper test";
    for wrong_str in [
        &b"AWSKMSEncryptionClient Frame"[..],
        &b"AWSKMSEncryptionClient Final Frame"[..],
        &b"AWSKMSEncryptionClient SingleBlock"[..], // close but missing space
        &b""[..],
    ] {
        let msg = build_nonframed_message_with_aad_overrides(
            pt,
            1,
            wrong_str,
            pt.len() as u64,
        );
        let res = try_decrypt_nonframed(&msg).await;
        assert!(
            res.is_err(),
            "nonframed message with AAD content {:?} must fail authentication, but decrypted to {:?}",
            std::str::from_utf8(wrong_str).unwrap_or("<non-utf8>"),
            res.ok()
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_nonframed_rejects_wrong_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Nonframed messages built with AAD content_length != plaintext length fail to decrypt — proves the decryptor uses the plaintext length exactly.
    //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
    let pt = b"nonframed length tamper test";
    for wrong_len in [0u64, (pt.len() as u64) + 1, (pt.len() as u64) - 1, u64::MAX] {
        let msg = build_nonframed_message_with_aad_overrides(
            pt,
            1,
            b"AWSKMSEncryptionClient Single Block",
            wrong_len,
        );
        let res = try_decrypt_nonframed(&msg).await;
        assert!(
            res.is_err(),
            "nonframed message with AAD content_length={wrong_len} must fail authentication, but decrypted to {:?}",
            res.ok()
        );
    }
}

// --- Framed tests (the real Rust encryptor produces framed ciphertexts) ---

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_matches_frame_sequence_number() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=Each frame's sequence-number field is written verbatim into the frame header; successful round-trip proves the AAD used the same sequence numbers.
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    let pt = vec![0xBBu8; 50];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // 50 bytes / 10 per frame = 5 frames; the last is the final frame.
    assert_eq!(frames.len(), 5, "expected 5 frames (4 regular + 1 final)");
    for (i, frame) in frames.iter().enumerate() {
        let expected_seq = (i + 1) as u32;
        assert_eq!(
            frame.0, expected_seq,
            "frame {i}: sequence number field must equal frame's position in sequence"
        );
    }
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves body AAD used matching frame sequence numbers");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_rejects_tampered_seq() {
    //= specification/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=Flipping a bit in the frame-header sequence-number field (not the content or tag) makes the decryptor reconstruct an AAD with the tampered seq, causing AES-GCM auth failure — proves the AAD used the frame's actual sequence number.
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    let pt = vec![0xCDu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    // Confirm baseline: untampered ciphertext decrypts.
    let ok = round_trip_framed(&pt, frame_length).await;
    assert_eq!(ok, pt, "baseline: untampered ciphertext must decrypt");

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
    assert!(
        res.is_err(),
        "tampering frame 1 sequence number from 1 to {tampered_seq} must cause authentication failure, but decryption succeeded"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_regular_frame_equals_frame_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Regular frames carry exactly frame_length encrypted bytes; successful round-trip proves the AAD used frame_length.
    //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of the [frame length](message-header.md#frame-length) field in the message header.
    let pt = vec![0xDDu8; 30];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // Regular frames: is_final=false.
    for (i, frame) in frames.iter().enumerate() {
        if !frame.4 {
            assert_eq!(
                frame.2.len() as u32, frame_length,
                "regular frame {i}: encrypted content length must equal frame_length"
            );
        }
    }
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used frame_length for regular frames");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_final_frame_bounded_by_frame_length() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=The final frame's explicit content_length field is bounded by frame_length; successful round-trip proves the AAD used that same bounded length.
    //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to 0 and less than or equal to the value of the [frame length](message-header.md#frame-length) field in the message header.
    let pt = vec![0xEEu8; 15];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let final_content_len = final_frame_content_length(&ct)
        .expect("ciphertext must contain a final frame");
    assert!(
        final_content_len <= frame_length,
        "final frame content_length ({final_content_len}) must be <= frame_length ({frame_length})"
    );
    // 15 bytes with frame_length=10 -> one regular frame (10) + final frame (5).
    assert_eq!(final_content_len, 5, "final frame should hold remaining 5 bytes");
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used bounded final-frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_content_length_framed_equals_per_frame_plaintext() {
    //= specification/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Sum of per-frame content lengths equals plaintext length; successful round-trip proves the AAD used per-frame plaintext lengths, not a single whole-message value.
    //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes, of the plaintext being encrypted in this frame.
    let pt = vec![0xCCu8; 25];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // Regular frames contribute frame_length; final frame contributes its remaining bytes.
    let total: usize = frames.iter().map(|f| f.2.len()).sum();
    assert_eq!(
        total, pt.len(),
        "sum of per-frame content lengths must equal plaintext length"
    );
    let decrypted = round_trip_framed(&pt, frame_length).await;
    assert_eq!(decrypted, pt, "round-trip proves AAD used per-frame plaintext lengths");
}
