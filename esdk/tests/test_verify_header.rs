// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#verify-the-header

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_header_fails_on_tampered_header() {
    let keyring = test_keyring().await;
    let plaintext = b"tamper header test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with a byte in the header body (byte 10 is safely within the header body).
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[10] ^= 0xFF;

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Untampered ct decrypts; tampered header byte → ValidationError, halting decrypt
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Tampered header body causes tag verify failure
    //# Once a valid message header is deserialized and decryption materials are available,
    //# this operation MUST validate the [message header body](../data-format/message-header.md#header-body)
    //# by using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# to decrypt with the following inputs:
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Tampering header body changes the AAD's header-body part → tag verify fails; EC-to-only-authenticate part of AAD is covered by test_verify_header_encryption_context_to_only_authenticate
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Tag verify failure proves tag from header is checked
    //# - the tag MUST be the value serialized in the message header's
    //# [authentication tag field](../data-format/message-header.md#authentication-tag)
    assert!(decrypt(&valid_input).await.is_ok(), "valid ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::ValidationError,
        "tampered header byte must produce ValidationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_header_encryption_context_to_only_authenticate() {
    let keyring = test_keyring().await;
    let plaintext = b"required ec test";

    // Create a required encryption context CMM that filters "keyA" out of the header
    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);
    let required_ec_keys = small_encryption_context_keys(SmallEncryptionContextVariation::A);
    let reproduced_ec = small_encryption_context(SmallEncryptionContextVariation::A);

    let default_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();

    let req_cmm = mpl()
        .create_required_encryption_context_cmm()
        .underlying_cmm(default_cmm)
        .required_encryption_context_keys(required_ec_keys)
        .send()
        .await
        .unwrap();

    let enc_input = EncryptInput::with_legacy_cmm(plaintext, encryption_context, req_cmm);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with the reproduced encryption context — proves the EC filtering is correct
    let dec_input = DecryptInput::with_legacy_keyring(&ct, reproduced_ec, keyring);
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Decrypt with reproduced EC succeeds; wrong filtering would fail header auth
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [decryption materials](../framework/structures.md#decryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [decryption material's](../framework/structures.md#decryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys-1)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    assert_eq!(result.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streamed_signed_output_not_signed_until_complete() {
    // Encrypt a multi-frame message with a signing suite, then tamper with
    // the signature (footer). decrypt_stream must return Err, proving that
    // output released before completion cannot be considered signed.
    let keyring = test_keyring().await;
    let plaintext = vec![0xBBu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut tampered_ct = valid_ct.clone();
    let len = tampered_ct.len();
    tampered_ct[len - 4] ^= 0xFF;

    let mut valid_cursor = std::io::Cursor::new(valid_ct.as_slice());
    let mut valid_output = Vec::new();
    let mut tampered_cursor = std::io::Cursor::new(tampered_ct.as_slice());
    let mut tampered_output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.unsafe_release_plaintext_before_verify = true;

    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Untampered ct streams to Ok; tampered signature → Err, so any output released before signature verification cannot be considered signed
    //# However, if the streamed Decrypt operation is using an algorithm suite with a signature algorithm
    //# all released output MUST NOT be considered signed data until
    //# this operation successfully completes.
    assert!(
        decrypt_stream(&mut valid_cursor, &mut valid_output, &stream_input).await.is_ok(),
        "valid ct must stream to Ok"
    );
    assert_eq!(
        decrypt_stream(&mut tampered_cursor, &mut tampered_output, &stream_input).await.unwrap_err().kind,
        ErrorKind::Esdk,
        "tampered signature must produce Esdk error"
    );
}

// Streaming decrypt output contains the parsed encryption context and
// algorithm suite ID after the operation completes successfully.
#[tokio::test(flavor = "multi_thread")]
async fn test_streamed_release_parsed_header_after_verification() {
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
    stream_input.unsafe_release_plaintext_before_verify = true;
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input)
        .await
        .unwrap();
    assert_eq!(output, plaintext);
    assert_eq!(
        result.encryption_context.get("release-key").map(String::as_str),
        Some("release-value"),
    );
    assert_eq!(
        result.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
    );
}

