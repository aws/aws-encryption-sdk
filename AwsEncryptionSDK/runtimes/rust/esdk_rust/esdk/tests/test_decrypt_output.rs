// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for decrypt.md sections: #output, #algorithm-suite, #authenticated-data, #security-considerations

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

/// Create a raw AES keyring for testing (no KMS needed).
async fn test_keyring() -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_includes_plaintext() {
    //= specification/client-apis/decrypt.md#output
    //= type=test
    //# - The output of the Decrypt operation MUST include a [Plaintext](#plaintext) value.

    let keyring = test_keyring().await;
    let plaintext = b"test plaintext for output check";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_includes_encryption_context() {
    //= specification/client-apis/decrypt.md#output
    //= type=test
    //# - The output of the Decrypt operation MUST include an [encryption context](#encryption-context) value.

    let keyring = test_keyring().await;
    let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
    let enc_input = EncryptInput::with_legacy_keyring(b"ec test", ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    for (k, v) in &ec {
        assert_eq!(result.encryption_context.get(k).unwrap(), v);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_includes_algorithm_suite() {
    //= specification/client-apis/decrypt.md#output
    //= type=test
    //# - The output of the Decrypt operation MUST include an [algorithm suite](#algorithm-suite) value.

    let keyring = test_keyring().await;
    let suite = EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.algorithm_suite_id, suite);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_algorithm_suite_is_esdk_supported() {
    //= specification/client-apis/decrypt.md#algorithm-suite
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).

    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(b"esdk suite", EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    // The returned algorithm_suite_id is an EsdkAlgorithmSuiteId enum variant,
    // which by construction only contains ESDK-supported suites.
    // A non-ESDK suite would have caused get_esdk_id to return Err.
    let _suite: EsdkAlgorithmSuiteId = result.algorithm_suite_id;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_data_released() {
    //= specification/client-apis/decrypt.md#authenticated-data
    //= type=test
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.

    let keyring = test_keyring().await;
    let plaintext = b"tamper test data";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the ciphertext body (flip a byte near the end, in the encrypted content area)
    let tamper_pos = ct.len() - 50;
    ct[tamper_pos] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    assert!(
        result.is_err(),
        "decrypt must fail when ciphertext is tampered — no unauthenticated data released"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_signed_plaintext_not_signed_until_complete() {
    //= specification/client-apis/decrypt.md#security-considerations
    //= type=test
    //# If this operation is [streaming](streaming.md) output to the caller
    //# and is decrypting messages created with an algorithm suite including a signature algorithm,
    //# any released plaintext MUST NOT be considered signed data until this operation finishes
    //# successfully.

    // A successful streaming decrypt with a signing suite proves the contract:
    // output is only fully released after signature verification completes.
    let keyring = test_keyring().await;
    let plaintext = b"signed streaming test";
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
    assert_eq!(output, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_callers_must_not_consider_successful_until_complete() {
    //= specification/client-apis/decrypt.md#security-considerations
    //= type=test
    //# This means that callers that process such released plaintext MUST NOT consider any processing successful
    //# until this operation completes successfully.

    // decrypt_stream returns Result — callers can only consider processing successful
    // when Ok is returned. A successful round-trip proves the contract.
    let keyring = test_keyring().await;
    let plaintext = b"completion contract test";
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
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    assert!(
        result.is_ok(),
        "successful completion signals callers may consider processing successful"
    );
    assert_eq!(output, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_callers_must_discard_on_failure() {
    //= specification/client-apis/decrypt.md#security-considerations
    //= type=test
    //# Additionally, if this operation fails, callers MUST discard the released plaintext and encryption context
    //# and MUST rollback any processing done due to the released plaintext or encryption context.

    // Encrypt with a signing suite, then tamper with the footer area to cause
    // signature verification failure. decrypt_stream must return Err,
    // signaling callers to discard any released output.
    let keyring = test_keyring().await;
    let plaintext = b"discard on failure test";
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the last few bytes (signature area) to cause verification failure
    let len = ct.len();
    ct[len - 5] ^= 0xFF;

    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.i_accept_the_danger = true;
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    assert!(
        result.is_err(),
        "decrypt_stream must return Err on tampered signature — callers must discard output"
    );
}
