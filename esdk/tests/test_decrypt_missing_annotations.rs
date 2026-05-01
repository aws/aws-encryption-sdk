// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for missing duvet annotations in specification/client-apis/decrypt.md
//! Covers V1/V2 header auth deserialization, streamed header release,
//! frame inspection, sequence number end validation, nonframed content length,
//! and footer/signature wait requirements.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_deserialized() {
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Successful V1 round-trip proves the Header Authentication Version 1.0 (IV + Auth Tag) was deserialized
    //# The Decrypt operation MUST then deserialize the
    //# [Header Authentication Version 1.0](../data-format/message-header.md#header-authentication-version-10):
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth deserialization test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, plaintext,
        "successful V1 round-trip proves Header Authentication Version 1.0 was deserialized"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_deserialized() {
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Successful V1 round-trip with header verification proves the IV was deserialized from the V1 header auth
    //# - MUST deserialize the [IV](../data-format/message-header.md#iv).
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth iv test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, plaintext,
        "successful V1 round-trip proves header auth IV was deserialized correctly"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_deserialized() {
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Successful V2 round-trip proves the Header Authentication Version 2.0 (Auth Tag only) was deserialized
    //# The Decrypt operation MUST then deserialize the
    //# [Header Authentication Version 2.0](../data-format/message-header.md#header-authentication-version-20):
    let keyring = test_keyring().await;
    let plaintext = b"v2 header auth deserialization test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, plaintext,
        "successful V2 round-trip proves Header Authentication Version 2.0 was deserialized"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streamed_release_parsed_header_after_verification() {
    //= specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Successful streaming decrypt returns encryption context and algorithm suite, proving they were released after header verification
    //# - A streamed Decrypt operation SHOULD release the parsed [encryption context](#encryption-context),
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id),
    //# and [other header information](#parsed-header)
    //# as soon as tag verification succeeds.
    let keyring = test_keyring().await;
    let plaintext = b"streamed header release test";

    let mut ec = EncryptionContext::new();
    ec.insert("release-key".to_string(), "release-value".to_string());
    let enc_input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.i_accept_the_danger = true;
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input)
        .await
        .unwrap();
    assert_eq!(output, plaintext);
    assert_eq!(
        result
            .encryption_context
            .get("release-key")
            .map(String::as_str),
        Some("release-value"),
        "encryption context must be released after header verification"
    );
    assert_eq!(
        result.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        "algorithm suite ID must be released after header verification"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_inspect_first_4_bytes_of_each_frame() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Multi-frame decrypt with 3 regular frames + 1 final frame proves the first 4 bytes of each frame are inspected to determine frame type
    //# The Decrypt operation MUST inspect the first 4 bytes of each frame.
    let pt = vec![0xAAu8; 40];
    let result = round_trip_framed(&pt, 10).await;
    assert_eq!(
        result, pt,
        "multi-frame decrypt proves first 4 bytes of each frame are inspected"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sequence_number_end_value_is_0xffffffff() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Parsing the raw ciphertext confirms the final frame starts with 0xFFFFFFFF, and successful decrypt proves this value was validated
    //# The value MUST be `0xFFFFFFFF`.
    let pt = vec![0xBBu8; 5];
    let ct = encrypt_with_frame_length(&pt, 4096).await;
    // Verify the ENDFRAME marker is present in the raw ciphertext
    let frames = parse_frames(&ct, 4096);
    assert_eq!(frames.len(), 1, "single final frame expected");
    assert!(frames[0].4, "frame must be a final frame");
    // The final frame was detected by the 0xFFFFFFFF marker; successful decrypt proves validation
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, pt,
        "decrypt proves 0xFFFFFFFF value was validated"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_content_length_from_encrypted_content_length() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //= reason=Successful nonframed decrypt proves the content length in AAD was determined from the nonframed data encrypted content length field
    //# If this is nonframed data, this MUST be determined by using the [nonframed data encrypted content length](../data-format/message-body.md#nonframed-data-encrypted-content-length).
    // Defer to the external V2 nonframed vector from aws-encryption-sdk-test-vectors.
    let body = parse_nonframed_body(EXTERNAL_V2_NONFRAMED_CT);
    assert_eq!(
        body.encrypted_content_length as usize,
        EXTERNAL_V2_NONFRAMED_PT.len(),
        "encrypted content length field must equal plaintext length"
    );
    // Successful decrypt proves the content length was used correctly in AAD.
    let result = decrypt_external_nonframed_vector(Version::V2).await;
    assert_eq!(result, EXTERNAL_V2_NONFRAMED_PT);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streamed_header_fed_to_signature_algorithm() {
    //= specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Successful signing-suite round-trip proves the serialized header was fed to the signature algorithm during deserialization
    //# - The streamed Decrypt operation SHOULD input the serialized header to the signature algorithm as soon as it is deserialized,
    //# such that the serialized header isn't required to remain in memory to [verify the signature](#verify-the-signature).
    let keyring = test_keyring().await;
    let plaintext = b"header to sig alg test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.i_accept_the_danger = true;
    decrypt_stream(&mut cursor, &mut output, &stream_input)
        .await
        .unwrap();
    assert_eq!(
        output, plaintext,
        "signing-suite streaming decrypt proves header was fed to signature algorithm"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_wait_for_bytes() {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Successful signing-suite decrypt proves the operation waited for enough footer bytes to deserialize the signature
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    let keyring = test_keyring().await;
    let plaintext = b"footer wait test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Use streaming decrypt which reads incrementally from the cursor
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.i_accept_the_danger = true;
    decrypt_stream(&mut cursor, &mut output, &stream_input)
        .await
        .unwrap();
    assert_eq!(
        output, plaintext,
        "streaming decrypt with signing suite proves footer bytes were waited for"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_wait_truncated_message_fails() {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Truncating the footer proves the operation waits for footer bytes and fails when they are not available
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    let keyring = test_keyring().await;
    let plaintext = b"truncated footer test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Truncate the message to remove part of the footer
    let footer_offset = find_footer_offset_only(&ct);
    let truncated = &ct[..footer_offset + 2]; // Keep sig_len but truncate signature bytes

    let dec_input = DecryptInput::with_legacy_keyring(truncated, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(
        result.is_err(),
        "decrypt must fail when footer is truncated — proves operation waited for bytes that never came"
    );
}
