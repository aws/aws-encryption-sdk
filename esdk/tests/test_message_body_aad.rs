// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/data-format/message-body-aad.md

mod test_helpers;

use aws_esdk::__test_internals::{BodyAADContent, body_aad};
use aws_esdk::{decrypt, DecryptInput, EncryptionContext, ErrorKind};
use test_helpers::*;

// Known literal values from the spec, repeated here verbatim so the tests
// assert against the specification text (not against the source constants).
const REGULAR_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Frame";
const FINAL_FRAME_STR: &[u8] = b"AWSKMSEncryptionClient Final Frame";
const SINGLE_BLOCK_STR: &[u8] = b"AWSKMSEncryptionClient Single Block";

#[test]
fn test_body_aad_structure_ordering() {
    //= spec/data-format/message-body-aad.md#structure
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
        //= spec/data-format/message-body-aad.md#message-id
        //= type=test
        //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
        //
        //= spec/data-format/message-body-aad.md#message-id
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
    //= spec/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
    //
    //= spec/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //= reason=The assertion compares raw bytes against a known UTF-8 literal; match proves the output is valid UTF-8
    //# The body AAD content value MUST be encoded as UTF-8 bytes.
    assert_eq!(
        &out[msg_id_16.len()..msg_id_16.len() + REGULAR_FRAME_STR.len()],
        REGULAR_FRAME_STR,
    );

    let mut out = Vec::new();
    body_aad(&msg_id_16, BodyAADContent::FinalFrame, 1, 0, &mut out);
    //= spec/data-format/message-body-aad.md#body-aad-content
    //= type=test
    //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
    assert_eq!(
        &out[msg_id_16.len()..msg_id_16.len() + FINAL_FRAME_STR.len()],
        FINAL_FRAME_STR,
    );

    let mut out = Vec::new();
    body_aad(&msg_id_32, BodyAADContent::SingleBlock, 1, 0, &mut out);
    //= spec/data-format/message-body-aad.md#body-aad-content
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
        //= spec/data-format/message-body-aad.md#sequence-number
        //= type=test
        //# The length of the sequence number field MUST be 4 bytes.
        //
        //= spec/data-format/message-body-aad.md#sequence-number
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
        //= spec/data-format/message-body-aad.md#content-length
        //= type=test
        //# The length of the content length field MUST be 8 bytes.
        //
        //= spec/data-format/message-body-aad.md#content-length
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

// ESDKs are forbidden from producing new nonframed messages,
// but are required to read existing nonframed messages.
// Use one reference nonframed message from test vectors for both V1 and V2 message formats.

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_nonframed_is_one() {
    for version in VERSIONS {
        let parsed = parse_external_nonframed_body(external_nonframed_ct(version), version);
        let iv_seq = u32::from_be_bytes([
            parsed.iv[8], parsed.iv[9], parsed.iv[10], parsed.iv[11],
        ]);
        // The message under test MUST have seq=1 already
        assert_eq!(iv_seq, 1, "{version:?}: body IV must encode seq=1");

        let pt = decrypt_external_nonframed_vector(version).await;
        //= spec/data-format/message-body-aad.md#sequence-number
        //= type=test
        //= reason=External vector's body IV encodes seq=1; decrypt succeeds, so AAD matched.
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
        // The message under test MUST have a valid length field already
        assert_eq!(
            parsed.encrypted_content_length,
            external_nonframed_pt(version).len() as u64,
            "{version:?}: encrypted-content-length field must equal plaintext length"
        );

        let pt = decrypt_external_nonframed_vector(version).await;
        //= spec/data-format/message-body-aad.md#content-length
        //= type=test
        //= reason=External vector's content-length field equals plaintext length; decrypt succeeds.
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
        let (start, expected_len) = match version {
            // V1 header: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16)
            Version::V1 => (4usize, 16usize),
            // V2 header: Version(1) + AlgSuiteID(2) + MessageID(32)
            Version::V2 => (3usize, 32usize),
        };
        // The message under test MUST have a valid length already
        assert!(
            ct.len() >= start + expected_len,
            "{version:?} header must be large enough to carry a {expected_len}-byte message ID"
        );

        let pt = decrypt_external_nonframed_vector(version).await;
        //= spec/data-format/message-body-aad.md#message-id
        //= type=test
        //= reason=V1 and V2 external vectors use 16- and 32-byte message IDs respectively; both decrypt.
        //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
        assert_eq!(
            pt, external_nonframed_pt(version),
            "{version:?}: decrypt output did not match plaintext — our AAD message ID length disagrees with the {expected_len}-byte header value"
        );
    }
}

// --- Negative nonframed test: tamper the external authority vector ---

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
        // Tamper: change the u64 length field from its true value to (value - 1).
        // The decryptor still reads a valid (length, content, tag) tuple (just
        // shifted by 1 byte), but the AAD it builds will carry the tampered length
        // — AES-GCM authentication then rejects it.
        let mut tampered = external_nonframed_ct(version).to_vec();
        let true_len = parsed.encrypted_content_length;
        let tampered_len = true_len - 1;
        tampered[len_offset..len_offset + 8]
            .copy_from_slice(&tampered_len.to_be_bytes());
        let err = try_decrypt_external_nonframed(version, &tampered)
            .await
            .expect_err(&format!(
                "{version:?}: decrypt must fail after tampering the nonframed encrypted_content_length field"
            ));
        //= spec/data-format/message-body-aad.md#content-length
        //= type=test
        //= reason=Tampered content_length causes AES-GCM auth failure.
        //# - For [nonframed data](message-body.md#nonframed-data), this value MUST equal the length, in bytes, of the plaintext data provided to the algorithm for encryption.
        assert_eq!(
            err.kind,
            ErrorKind::CryptographicError,
            "{version:?}: expected ErrorKind::CryptographicError (AES-GCM tag mismatch), got {err:?}"
        );
        let err_str = err.to_string();
        assert!(
            err_str.contains("Cryptographic"),
            "{version:?}: expected display message to contain \"Cryptographic\", got: {err_str}"
        );
    }
}

// --- Framed tests ---

#[tokio::test(flavor = "multi_thread")]
async fn test_body_aad_sequence_number_framed_matches_frame_sequence_number() {
    let pt = vec![0xBBu8; 50];
    let frame_length = 10u32;
    let ct = encrypt_with_frame_length(&pt, frame_length).await;
    let frames = parse_frames(&ct, frame_length);
    // 50 bytes / 10 per frame = 5 frames; the last is the final frame.
    assert_eq!(frames.len(), 5, "expected 5 frames (4 regular + 1 final)");
    for (i, frame) in frames.iter().enumerate() {
        let expected_seq = (i + 1) as u32;
        //= spec/data-format/message-body-aad.md#sequence-number
        //= type=test
        //= reason=Frame headers carry 1..N; companion test proves AAD uses them.
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
    let err = decrypt(&dec_input)
        .await
        .expect_err("tampering frame 1's sequence number must cause decrypt to fail");
    //= spec/data-format/message-body-aad.md#sequence-number
    //= type=test
    //= reason=Tampered frame sequence number is rejected by the decryptor.
    //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
    assert_eq!(
        err.kind,
        ErrorKind::SerializationError,
        "expected ErrorKind::SerializationError after tampering frame 1's seq from 1 to {tampered_seq}, got {err:?}"
    );
    let err_str = err.to_string();
    assert!(
        err_str.contains("Sequence number out of order"),
        "expected display message to contain \"Sequence number out of order\" after tampering frame 1's seq from 1 to {tampered_seq}, got: {err_str}"
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
            //= spec/data-format/message-body-aad.md#content-length
            //= type=test
            //= reason=Each regular frame's encrypted content is frame_length bytes; round-trip corroborates AAD.
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
    //= spec/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Final frame's content_length lies in [0, frame_length]; round-trip corroborates.
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
    //= spec/data-format/message-body-aad.md#content-length
    //= type=test
    //= reason=Per-frame content lengths sum to plaintext length; round-trip corroborates.
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
