// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#decrypt-the-message-body

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_validation() {
    // Encrypt a message, then tamper with the final frame's content length field
    // to exceed the frame length. Decrypt must fail.
    let pt = vec![0xEEu8; 5];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;

    // First, verify the untampered content_length is <= frame_length on wire
    let frames = parse_all_frames(&ct, 10);
    let final_frame = frames.last().unwrap();
    assert!(final_frame.is_final);
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire content_length (5) <= frame_length (10) verified directly
    //# MUST ensure that the length of the encrypted content field is
    //# less than or equal to the frame length deserialized in the message header.
    assert!(
        final_frame.content_length <= 10,
        "untampered: content_length {} must be <= frame_length 10",
        final_frame.content_length
    );

    // Valid value decrypts as expected
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "valid content_length decrypts");

    // Now tamper: set content_length to 11 (exceeds frame_length=10)
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == ENDFRAME_MARKER {
            let bad_len = 11u32.to_be_bytes();
            ct[i + 20..i + 24].copy_from_slice(&bad_len);
            break;
        }
    }
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=content_length > frame_length → SerializationError; proves the check exists
    //# If this is a final frame, this MUST be determined by using the [final frame encrypted content length](../data-format/message-body.md#final-frame-encrypted-content-length).
    let err = decrypt(&dec_input).await.expect_err("content_length > frame_length must fail");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_on_tampered_auth_tag() {
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "baseline must pass");
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    ct[tag_end] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered auth tag must fail");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered auth tag → CryptographicError proves AEAD check runs
    //# If this decryption fails, this operation MUST immediately halt and fail.
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_fields_deserialized_from_wire() {
    // Directly verify each frame field is present at the correct wire offset.
    // 30 bytes / frame_length=10 → 2 regular + 1 final = 3 frames.
    let pt = vec![0xABu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames.len(), 3, "expected 3 frames (2 regular + 1 final)");

    // Regular frame 1: verify deserialized fields on wire
    let f = &frames[0];
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire seq_num at expected offset proves it was deserialized
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    assert_eq!(f.seq_num, 1, "frame 1 seq_num must be 1");
    assert_eq!(f.seq_num_bytes.len(), 4, "seq_num is 4 bytes on wire");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire IV at expected offset proves it was deserialized
    //# - MUST deserialize the [IV](../data-format/message-body.md#regular-frame-iv).
    assert_eq!(f.iv.len(), IV_LEN, "IV is 12 bytes on wire");
    assert_eq!(f.iv_offset, f.seq_num_offset + 4, "IV follows seq_num");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire content at expected offset proves it was deserialized
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
    assert_eq!(f.content.len(), 10, "regular frame content is frame_length bytes");
    assert_eq!(f.content_offset, f.iv_offset + IV_LEN, "content follows IV");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire tag at expected offset proves it was deserialized
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
    assert_eq!(f.tag.len(), TAG_LEN, "auth tag is 16 bytes on wire");
    assert_eq!(f.tag_offset, f.content_offset + 10, "tag follows content");

    // Final frame: verify content_length field
    let ff = &frames[2];
    assert!(ff.is_final, "frame 3 must be final");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire content_length field in final frame proves it was deserialized
    //# - MUST deserialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    assert_eq!(ff.content_length_bytes.unwrap().len(), 4, "content_length is 4 bytes");
    assert_eq!(ff.content_length, 10, "final frame content = remaining 10 bytes");

    // Round-trip corroboration: all fields consumed correctly
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_plaintext_released() {
    // Tamper with encrypted content in the first frame. Decrypt must fail
    // and return no plaintext at all.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Baseline: untampered ciphertext decrypts successfully.
    assert_eq!(decrypt_ciphertext(&ct).await.plaintext, pt, "baseline must pass");
    // Tamper with a byte in the encrypted content of the first regular frame
    let tamper_offset = body_start + 4 + IV_LEN + 1;
    ct[tamper_offset] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered ciphertext must produce error, not partial plaintext");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered body returns Err; no DecryptOutput means no plaintext released
    //# This operation MUST NOT release any unauthenticated plaintext.
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_held_until_signature_verification() {
    // Encrypt with a signing algorithm suite, then tamper with the signature.
    // Decrypt must fail, proving the final frame plaintext was held back
    // pending signature verification and never released.
    let keyring = test_keyring().await;
    let pt = vec![0xABu8; 16];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(4096).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;
    // Baseline: untampered ciphertext decrypts successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline must pass before tamper");
    // Tamper with the last byte of the signature to cause verification failure
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered signature must cause decrypt failure, proving final frame was held back");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Tampered signature → Err; final frame plaintext never reaches caller
    //# Any plaintext decrypted from [nonframed data](../data-format/message-body.md#nonframed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_fails_on_tampered_auth_tag() {
    // Tamper with the authentication tag in the external V2 nonframed vector. Decrypt must fail.
    let mut ct = EXTERNAL_V2_NONFRAMED_CT.to_vec();
    // Baseline: untampered vector decrypts successfully.
    let baseline = try_decrypt_external_nonframed(Version::V2, &ct).await;
    assert!(baseline.is_ok(), "baseline external vector must decrypt");
    // The auth tag is the last 16 bytes of the message
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let result = try_decrypt_external_nonframed(Version::V2, &ct).await;
    let err = result.expect_err("tampered nonframed auth tag must cause immediate decryption failure");
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=Tampered nonframed auth tag → CryptographicError, proving AEAD halts on failure
    //# If this decryption fails, this operation MUST immediately halt and fail.
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_value_is_0xffffffff() {
    let pt = vec![0xBBu8; 5];
    let ct = encrypt_with_frame_length(&pt, 4096).await;
    let frames = parse_all_frames(&ct, 4096);
    assert_eq!(frames.len(), 1, "single final frame expected");
    let f = &frames[0];
    assert!(f.is_final, "frame must be a final frame");

    // Byte-level check: the ENDFRAME marker is 0xFFFFFFFF at the frame start
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire bytes at frame start are exactly 0xFF 0xFF 0xFF 0xFF
    //# The value MUST be `0xFFFFFFFF`.
    let marker = f.endframe_marker_bytes.expect("final frame must have endframe marker");
    assert_eq!(marker, &[0xFF, 0xFF, 0xFF, 0xFF], "endframe marker must be 0xFFFFFFFF byte-by-byte");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Endframe marker field present at frame_offset on wire
    //# - MUST deserialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
    assert_eq!(f.endframe_marker_offset, Some(f.frame_offset), "endframe marker at frame start");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Seq num field at expected offset, value=1 for single final frame
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    assert_eq!(f.seq_num, 1, "final frame seq_num must be 1");
    assert_eq!(f.seq_num_offset, f.frame_offset + 4, "seq_num follows endframe marker");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=IV field at expected offset, 12 bytes
    //# - MUST deserialize the [IV](../data-format/message-body.md#final-frame-iv).
    assert_eq!(f.iv.len(), IV_LEN, "IV is 12 bytes");
    assert_eq!(f.iv_offset, f.seq_num_offset + 4, "IV follows seq_num");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Content length field at expected offset, 4 bytes, value=5
    //# - MUST deserialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    let cl_bytes = f.content_length_bytes.expect("final frame has content_length field");
    assert_eq!(cl_bytes.len(), 4, "content_length is 4 bytes on wire");
    assert_eq!(f.content_length, 5, "content_length = plaintext length for final frame");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Encrypted content at expected offset, length matches content_length field
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    assert_eq!(f.content.len(), 5, "encrypted content = content_length bytes");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Auth tag at expected offset, 16 bytes
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    assert_eq!(f.tag.len(), TAG_LEN, "auth tag is 16 bytes");
    assert_eq!(f.tag_offset, f.content_offset + 5, "tag follows content");

    // Round-trip corroboration
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_first_frame_sequence_number_is_one() {
    // Parse the on-wire ciphertext and verify the first frame's sequence number is 1.
    let pt = vec![0xAAu8; 30];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert!(frames.len() >= 2, "need at least 2 frames");
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire first frame seq_num == 1 proves requirement
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    //
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Wire parse extracts seq_num from first 4 bytes, proving deserialization
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    assert_eq!(frames[0].seq_num, 1, "first frame sequence number must be 1 on the wire");
    // Round-trip corroboration
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_sequence_numbers_increment() {
    // 50 bytes at frame_length=10 → 4 regular + 1 final = 5 frames.
    let pt = vec![0xBBu8; 50];
    let ct = encrypt_with_frame_length(&pt, 10).await;
    let frames = parse_all_frames(&ct, 10);
    assert_eq!(frames.len(), 5, "50 bytes / 10-byte frames = 4 regular + 1 final");

    // Verify regular frames have incrementing sequence numbers on wire
    for (i, frame) in frames[..4].iter().enumerate() {
        assert!(!frame.is_final, "frame {i} must be regular");
        assert_eq!(frame.seq_num, (i + 1) as u32, "frame {i}: seq_num on wire");
        // Byte-level: seq_num bytes encode the expected value in big-endian
        let expected_bytes = ((i + 1) as u32).to_be_bytes();
        assert_eq!(frame.seq_num_bytes, &expected_bytes, "frame {i}: seq_num bytes");
    }

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire seq_nums 1..5 verified byte-by-byte across 5 frames
    //# Otherwise, this value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    assert_eq!(frames[4].seq_num, 5, "final frame seq_num must be 5");

    // Verify final frame has endframe marker on wire
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Final frame's first 4 bytes are 0xFFFFFFFF on wire
    //# If the first 4 bytes have a value of 0xFFFFFFFF,
    //# the Decrypt operation MUST treat them as the [Sequence Number End](../data-format/message-body.md#sequence-number-end)
    //# and deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
    let marker = frames[4].endframe_marker_bytes.expect("final frame has marker");
    assert_eq!(marker, &[0xFF, 0xFF, 0xFF, 0xFF]);

    // Regular frames: first 4 bytes are NOT 0xFFFFFFFF
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Regular frames' first 4 bytes != 0xFFFFFFFF, treated as seq num
    //# Otherwise, the Decrypt operation MUST treat them as the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number)
    //# and deserialize the following bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
    for frame in &frames[..4] {
        assert!(frame.endframe_marker_bytes.is_none(), "regular frame must not have endframe marker");
    }

    // Round-trip corroboration
    let result = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_body_fields_deserialized_from_wire() {
    // Parse the external V2 nonframed vector directly and verify each field.
    let body = parse_external_nonframed_body(EXTERNAL_V2_NONFRAMED_CT, Version::V2);

    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector's IV field is 12 bytes at expected offset
    //# - The IV MUST be the [IV](../data-format/message-body.md#nonframed-data-iv) deserialized from the message body.
    assert_eq!(body.iv.len(), IV_LEN, "nonframed IV must be 12 bytes");

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Encrypted content length field equals expected plaintext length (10240)
    //# If this is nonframed data, this MUST be determined by using the [nonframed data encrypted content length](../data-format/message-body.md#nonframed-data-encrypted-content-length).
    assert_eq!(
        body.encrypted_content_length as usize,
        EXTERNAL_V2_NONFRAMED_PT.len(),
        "encrypted content length must equal plaintext length"
    );

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=On-wire seq_num for nonframed is always 1 (checked via AAD construction)
    //# If this is nonframed data, this value MUST be 1.
    // The IV's last 4 bytes encode the sequence number for nonframed data.
    let iv_seq = u32::from_be_bytes([body.iv[8], body.iv[9], body.iv[10], body.iv[11]]);
    assert_eq!(iv_seq, 1, "nonframed IV sequence number must be 1");

    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //= reason=External vector decrypts to known plaintext; proves all AES-GCM inputs correct
    //# If a message has the [nonframed](../data-format/message-body.md#nonframed-data) content type,
    //# the Decrypt operation MUST deserialize the message body according to the
    //# [nonframed data specification](../data-format/message-body.md#nonframed-data)
    //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}
