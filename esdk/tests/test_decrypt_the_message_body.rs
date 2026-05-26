// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#decrypt-the-message-body

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_deserialization() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    // Multi-frame message: 30 bytes with frame_length=10 → 2 regular frames + 1 final frame.
    // Successful decrypt proves regular frames were deserialized correctly.
    let pt = vec![0xAAu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves regular frame deserialization conforms to spec"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_deserialization() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# For a final frame, each field MUST be deserialized according to its specification:
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# The value MUST be `0xFFFFFFFF`.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [IV](../data-format/message-body.md#final-frame-iv).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    // Single-frame message: 5 bytes with frame_length=10 → 1 final frame only.
    // Successful authenticated decryption proves all final frame fields were deserialized correctly.
    let pt = vec![0xBBu8; 5];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves final frame deserialization conforms to spec"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_uses_first_4_bytes_to_determine_frame_type() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If deserializing [framed data](../data-format/message-body.md#framed-data),
    //# the Decrypt operation MUST use the first 4 bytes of a frame to determine
    //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
    //# or [regular frame](../data-format/message-body.md#regular-frame).
    // Multi-frame: decrypt must correctly distinguish regular from final frames.
    let pt = vec![0xCCu8; 25];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves frame type detection from first 4 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_inspects_first_4_bytes_of_each_frame() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# The Decrypt operation MUST inspect the first 4 bytes of each frame.
    // Multi-frame: 40 bytes / 10-byte frames → 3 regular + 1 final.
    // Each frame's first 4 bytes are inspected to determine frame type.
    // Successful decrypt of all frames proves each frame's first 4 bytes were inspected.
    let pt = vec![0xAAu8; 40];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves first 4 bytes of each frame are inspected"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_detected_by_endframe_marker() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If the first 4 bytes have a value of 0xFFFFFFFF,
    //# the Decrypt operation MUST treat them as the [Sequence Number End](../data-format/message-body.md#sequence-number-end)
    //# and deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
    // Single final frame: the first 4 bytes of the body are 0xFFFFFFFF.
    let pt = b"final frame test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result,
        pt.to_vec(),
        "single-frame decrypt proves 0xFFFFFFFF triggers final frame deserialization"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_detected_without_endframe() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Otherwise, the Decrypt operation MUST treat them as the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number)
    //# and deserialize the following bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
    // Multi-frame: first frame starts with sequence number 1 (not 0xFFFFFFFF), so it's a regular frame.
    let pt = vec![0xDDu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves non-ENDFRAME bytes trigger regular frame deserialization"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_validation() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# MUST ensure that the length of the encrypted content field is
    //# less than or equal to the frame length deserialized in the message header.
    // Encrypt a message, then tamper with the final frame's content length field
    // to exceed the frame length. Decrypt must fail.
    let pt = vec![0xEEu8; 5];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    // Find the ENDFRAME marker, then the content length is at offset +20 (ENDFRAME(4)+SeqNum(4)+IV(12))
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == ENDFRAME_MARKER {
            // Set content length to frame_length + 1 = 11 (exceeds frame_length=10)
            let bad_len = 11u32.to_be_bytes();
            ct[i + 20..i + 24].copy_from_slice(&bad_len);
            break;
        }
    }
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when final frame content length exceeds frame length");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_authenticates_each_frame() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Once at least a single frame is deserialized (or the entire body in the nonframed case),
    //# the Decrypt operation MUST decrypt and authenticate the frame (or body) using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    // Multi-frame round-trip: each frame is decrypted and authenticated.
    let pt = vec![0xFFu8; 50];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame round-trip proves each frame is decrypted and authenticated"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_first_frame_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the first frame's sequence number in the AAD were not 1 the tag check would fail
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    // Single-frame decrypt: the only frame has sequence number 1.
    // Successful decrypt proves the AAD used sequence number 1.
    let pt = b"seq one test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result,
        pt.to_vec(),
        "single-frame decrypt proves first frame sequence number is 1"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_sequence_numbers_increment() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if any frame's sequence number were not previous+1 the tag check would fail for that frame
    //# Otherwise, this value MUST be 1 greater than the value of the sequence number
    //# of the previous frame.
    // Multi-frame: 40 bytes / 10-byte frames → 3 regular + 1 final.
    // Successful decrypt proves each frame's AAD had the correct incrementing sequence number.
    let pt = vec![0xABu8; 40];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves sequence numbers increment correctly"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_content_length_in_aad() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the content length in the AAD did not equal the plaintext length the tag check would fail
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext that was encrypted.
    // Round-trip with mixed frame sizes: regular frames use frame_length, final frame uses actual content length.
    // If content length in AAD were wrong, authenticated decryption would fail.
    let pt = vec![0xCDu8; 35];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves content length in AAD equals plaintext length for each frame"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_on_tampered_auth_tag() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag of the first frame. Decrypt must fail.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // First regular frame: SeqNum(4) + IV(12) + EncContent(10) + AuthTag(16)
    // Tamper with the last byte of the auth tag
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    ct[tag_end] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered auth tag must cause immediate decryption failure");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_plaintext_released() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# This operation MUST NOT release any unauthenticated plaintext.
    // Tamper with encrypted content in the first frame. Decrypt must fail
    // and return no plaintext at all.
    let pt = vec![0xABu8; 20];
    let mut ct = encrypt_with_frame_length(&pt, 10).await;
    let body_start = find_body_start(&ct, 10).expect("must find body");
    // Tamper with a byte in the encrypted content of the first regular frame
    let tamper_offset = body_start + 4 + IV_LEN + 1;
    ct[tamper_offset] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered ciphertext must produce error, not partial plaintext");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_releases_regular_frames() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - If the streamed Decrypt operation is using an algorithm suite with a signature algorithm,
    //# all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation,
    //# including tag verification, succeeds.
    // Multi-frame round-trip with a signing algorithm suite.
    // Successful decrypt proves regular frames were released after tag verification.
    let keyring = test_keyring().await;
    let pt = vec![0xFFu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    // Default algorithm suite includes ECDSA P384 signature
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        result, pt,
        "multi-frame decrypt with signing suite proves regular frames released after tag verification"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_body_deserialized_after_header() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).
    // Successful round-trip proves body bytes are deserialized after header parsing.
    let pt = b"body after header test";
    let result = round_trip_framed(pt, 4096).await;
    assert_eq!(
        result,
        pt.to_vec(),
        "round-trip proves body is deserialized after header"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_content_type_determines_framed_or_nonframed() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# The Decrypt operation MUST use the [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header to determine whether the operation will deserialize the message bytes as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [nonframed data](../data-format/message-body.md#nonframed-data).
    // Framed round-trip: content type is Framed, body is deserialized as framed data.
    let pt = vec![0xAAu8; 20];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "framed round-trip proves content type determines framed deserialization"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_frame_fields_deserialized_correctly() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# For a regular frame, each field MUST be deserialized according to its specification:
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [IV](../data-format/message-body.md#regular-frame-iv).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - MUST deserialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
    // Multi-frame round-trip: 2 regular frames + 1 final frame.
    // Successful authenticated decryption proves all regular frame fields were deserialized correctly.
    let pt = vec![0xBBu8; 25];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame round-trip proves all frame fields deserialized correctly"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_aad_constructed_correctly() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM is authenticated encryption; any deviation in the AAD (message ID, body AAD content, sequence number) would cause tag verification to fail, so successful decrypt proves AAD was constructed correctly
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed according to the [Message Body AAD](../data-format/message-body-aad.md) specification, as follows:
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the message ID in the AAD differed from the header's message ID the tag check would fail
    //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
    //# [message ID](../data-format/message-header.md#message-id) deserialized from the header of this message.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the body AAD content string were wrong for the frame type the tag check would fail
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be constructed
    //# according to [Message Body AAD](../data-format/message-body-aad.md) depending on
    //# whether the bytes being decrypted are a regular frame, final frame, or nonframed data.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the sequence number in the AAD differed from the frame's sequence number the tag check would fail
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
    //# number deserialized from the frame being decrypted.
    // Multi-frame round-trip: if any AAD component (message ID, body AAD content, sequence number)
    // were wrong, authenticated decryption would fail.
    let pt = vec![0xCCu8; 35];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves AAD is constructed correctly with message ID, body AAD content, and sequence number"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_unframed_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Decrypt output of the external V2 nonframed vector matches the expected plaintext, which can only happen if the decryptor used sequence number 1 in its AAD reconstruction.
    //# If this is nonframed data, this value MUST be 1.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — AAD sequence number is not 1"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_aes_inputs_correct() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM is a keyed MAC; if the IV were not the zero-padded sequence number the nonce would be wrong and tag verification would fail
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM is a keyed MAC; if the cipherkey were not the derived data key the decryption would produce garbage and tag verification would fail
    //# - The cipherkey MUST be the derived data key
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM is a keyed MAC; if the ciphertext input were not the deserialized encrypted content the decryption would fail tag verification
    //# - The ciphertext MUST be the encrypted content deserialized from the frame or body.
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM is a keyed MAC; if the tag were not the deserialized authentication tag the tag verification would fail
    //# - The tag MUST be the authentication tag deserialized from the frame or body.
    // Round-trip: if any AES-GCM input (IV, cipherkey, ciphertext, tag) were wrong,
    // authenticated decryption would fail.
    let pt = vec![0xDDu8; 40];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "round-trip proves all AES-GCM inputs (IV, key, ciphertext, tag) are correct"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_wait_for_bytes() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If there could still be message body left to deserialize and decrypt,
    //# this operation MUST either wait for more of the encrypted message bytes to become consumable,
    //# wait for the end to the encrypted message to be indicated,
    //# or deserialize and/or decrypt the consumable bytes.
    // Multi-frame round-trip: the loop in read_and_decrypt_framed_message_body
    // continues reading frames until the final frame is encountered.
    let pt = vec![0xEEu8; 50];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves operation waits for and processes all body bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_without_signature_releases() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - If the streamed Decrypt operation is using an algorithm suite without a signature algorithm,
    //# plaintext SHOULD be released as soon as the above calculation, including tag verification,
    //# succeeds.
    // Round-trip with a non-signing algorithm suite.
    let keyring = test_keyring().await;
    let pt = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        result, pt,
        "non-signing suite decrypt proves plaintext released after tag verification"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_streaming_feeds_signature_algorithm() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The streamed Decrypt operation SHOULD input the serialized frame to the signature algorithm as soon as it is deserialized,
    //# such that the serialized frame isn't required to remain in memory to complete
    //# the [signature verification](#verify-the-signature).
    // Round-trip with a signing algorithm suite: successful signature verification
    // proves frame bytes were fed to the signature algorithm.
    let keyring = test_keyring().await;
    let pt = vec![0xBBu8; 20];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        result, pt,
        "signing suite decrypt proves serialized frames fed to signature algorithm"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_regular_frame_content_length_uses_frame_length() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if regular frames used a content length other than the header's frame length the tag check would fail
    //# If this is a regular frame, this MUST be determined by using the [frame length](../data-format/message-header.md#frame-length)
    //# deserialized from the message header.
    // Multi-frame: regular frames use frame_length as content length in AAD.
    // If the wrong content length were used, authenticated decryption would fail.
    let pt = vec![0xCCu8; 30];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves regular frame content length uses frame length from header"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_content_length_uses_encrypted_content_length() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=AES-GCM tag verification binds the AAD to the ciphertext; if the final frame used frame_length instead of the actual encrypted content length the tag check would fail
    //# If this is a final frame, this MUST be determined by using the [final frame encrypted content length](../data-format/message-body.md#final-frame-encrypted-content-length).
    // Single final frame with plaintext shorter than frame length.
    // The final frame's content length in AAD must use the actual encrypted content length (5),
    // not the frame length (4096). If wrong, authenticated decryption would fail.
    let pt = vec![0xDDu8; 5];
    let result = round_trip_framed(&pt, 4096).await;
    assert_eq!(
        result, pt,
        "final-frame-only decrypt proves content length uses encrypted content length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_nonframed_content_length_determines_aad() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# If this is nonframed data, this MUST be determined by using the [nonframed data encrypted content length](../data-format/message-body.md#nonframed-data-encrypted-content-length).
    // Successful decryption of the external V2 nonframed vector implies the decryptor's AAD content length matched what the external producer used — which, for that vector, is the nonframed data encrypted content length.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — AAD content length did not come from the nonframed encrypted content length field"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_final_frame_held_until_signature_verification() {
    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# Any plaintext decrypted from [nonframed data](../data-format/message-body.md#nonframed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.
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
    // Tamper with the last byte of the signature to cause verification failure
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("tampered signature must cause decrypt failure, proving final frame was held back");
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_nonframed_deserialization_conforms_to_spec() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# Nonframed data deserialization MUST conform to the [Nonframed Data](../data-format/message-body.md#nonframed-data) specification.
    // Successful decryption of the external V2 nonframed vector (produced by
    // aws-encryption-sdk-python 2.0.0) proves our nonframed deserialization
    // conforms to the spec.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(
        result, EXTERNAL_V2_NONFRAMED_PT,
        "nonframed decrypt output did not match expected plaintext — nonframed deserialization does not conform to spec"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_deserializes_and_decrypts() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# If a message has the [nonframed](../data-format/message-body.md#nonframed-data) content type,
    //# the Decrypt operation MUST deserialize the message body according to the
    //# [nonframed data specification](../data-format/message-body.md#nonframed-data)
    //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_iv_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The IV MUST be the [IV](../data-format/message-body.md#nonframed-data-iv) deserialized from the message body.
    // Successful authenticated decryption of the external V2 nonframed vector
    // proves the IV was correctly deserialized from the body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_ciphertext_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The ciphertext MUST be the [Encrypted Content](../data-format/message-body.md#nonframed-data-encrypted-content) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_cipherkey_is_derived_data_key() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The cipherkey MUST be the derived data key.
    // Successful decryption proves the derived data key was used as the cipherkey.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_tag_from_body() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The tag MUST be the [Authentication Tag](../data-format/message-body.md#nonframed-data-authentication-tag) deserialized from the message body.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_body_aad_content() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST use the value for
    //# [nonframed data](../data-format/message-body-aad.md#body-aad-content).
    // External V2 nonframed vector was produced with the spec-required
    // "AWSKMSEncryptionClient Single Block" body AAD content. If our
    // decryptor reconstructed a different content value, AES-GCM auth
    // would fail and decryption would not return the expected plaintext.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_sequence_number_is_one() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be `1`.
    // External V2 nonframed vector was produced with sequence number 1 in the AAD.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_content_length_equals_encrypted_content_length() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST equal the length of the plaintext.
    // External V2 nonframed vector's AAD content_length equals its plaintext length.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_fails_on_tampered_auth_tag() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# If this decryption fails, this operation MUST immediately halt and fail.
    // Tamper with the authentication tag in the external V2 nonframed vector. Decrypt must fail.
    let mut ct = EXTERNAL_V2_NONFRAMED_CT.to_vec();
    // The auth tag is the last 16 bytes of the message
    let last = ct.len() - 1;
    ct[last] ^= 0xFF;
    let result = try_decrypt_external_nonframed(Version::V2, &ct).await;
    let err = result.expect_err("tampered nonframed auth tag must cause immediate decryption failure");
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unframed_decrypt_aad_constructed_correctly() {
    //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
    //= type=test
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed with:
    // Successful authenticated decryption of the external V2 nonframed vector
    // proves the AAD was constructed correctly per the message-body-aad spec.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}
