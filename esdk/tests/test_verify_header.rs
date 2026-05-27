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
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with a byte in the header body (byte 10 is safely within the header body)
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[10] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.
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
    //= reason=Tag verify failure proves the cipherkey, tag, and AAD inputs are used
    //# - the cipherkey MUST be the derived data key
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Tag verify failure proves tag from header is checked
    //# - the tag MUST be the value serialized in the message header's
    //# [authentication tag field](../data-format/message-header.md#authentication-tag)
        let err = result.expect_err("decrypt must fail when header bytes are tampered (tag verification failure)");
    assert_eq!(err.kind, ErrorKind::ValidationError, "got: {err:?}");
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
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [decryption materials](../framework/structures.md#decryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [decryption material's](../framework/structures.md#decryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys-1)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, plaintext,
        "successful round-trip with required EC keys proves encryption context to only authenticate is correctly filtered and serialized"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streamed_signed_output_not_signed_until_complete() {
    // Encrypt a multi-frame message with a signing suite, then tamper with the
    // signature (footer). decrypt_stream must return Err, proving that output
    // released before completion cannot be considered signed.
    let keyring = test_keyring().await;
    let plaintext = vec![0xBBu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the footer (signature) to cause verification failure
    let len = ct.len();
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[len - 4] ^= 0xFF;

    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.unsafe_release_plaintext_before_verify = true;
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# However, if the streamed Decrypt operation is using an algorithm suite with a signature algorithm
    //# all released output MUST NOT be considered signed data until
    //# this operation successfully completes.
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    let err = result.expect_err("streaming decrypt must fail on tampered signature — output was not signed data");
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

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
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Streaming decrypt output includes EC and suite, proving release after verification
    //# - A streamed Decrypt operation SHOULD release the parsed [encryption context](#encryption-context),
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id),
    //# and [other header information](#parsed-header)
    //# as soon as tag verification succeeds.
    assert_eq!(
        result.encryption_context.get("release-key").map(String::as_str),
        Some("release-value"),
    );
    assert_eq!(
        result.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
    );
}

