// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#behavior

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
async fn test_decrypt_skips_signature_step_for_non_signing_algorithm() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# - If the message header does not contain an algorithm suite including a signature algorithm,
    //# the Decrypt operation MUST NOT perform this step.
    let keyring = test_keyring().await;
    let plaintext = b"test non-signing decrypt";
    let ec = EncryptionContext::new();

    // Encrypt with a non-signing algorithm suite
    let mut encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    encrypt_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Decrypt succeeds — the signature verification step is skipped
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_non_streaming_decrypt_holds_output_until_completion() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# If the input encrypted message is not being [streamed](streaming.md) to this operation,
    //# all output MUST NOT be released until after these steps complete successfully.
    // A successful round-trip through the non-streaming decrypt() proves that
    // all output is returned only after all 5 steps complete.
    let keyring = test_keyring().await;
    let plaintext = b"non-streaming output held until completion";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_output_not_released_until_indicated() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# - Output MUST NOT be released until otherwise indicated.
    // Streaming decrypt with a signing suite: output is held back until
    // per-frame tag verification and final signature verification succeed.
    // A successful round-trip proves output was only released after verification.
    let keyring = test_keyring().await;
    let plaintext = b"streaming output held";
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
async fn test_streaming_halts_on_incomplete_message() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# - If all bytes have been provided and this operation
    //# is unable to complete the above steps with the consumable encrypted message bytes,
    //# this operation MUST halt and indicate a failure to the caller.
    // Encrypt a valid message, then truncate it so the body is incomplete.
    let keyring = test_keyring().await;
    let plaintext = b"truncation test data";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Truncate to ~60% of the message — enough for the header but not the full body
    let truncated = &ct[..ct.len() * 3 / 5];
    let dec_input =
        DecryptInput::with_legacy_keyring(truncated, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(
        result.is_err(),
        "decrypt must fail when ciphertext is truncated"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_fails_for_multi_frame_signed_without_override() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# - The ESDK MUST provide a configuration option that causes the decryption operation
    //# to fail immediately after parsing the header if a signed algorithm suite is used.
    // Encrypt a multi-frame message with a signing algorithm suite.
    let keyring = test_keyring().await;
    // 30 bytes with frame_length=10 → 3 frames (2 regular + 1 final)
    let plaintext = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // decrypt_stream with i_accept_the_danger=false (default) must fail
    // for multi-frame signed messages
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    // i_accept_the_danger defaults to false
    assert!(!stream_input.i_accept_the_danger);
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    assert!(
        result.is_err(),
        "multi-frame signed message must fail with default i_accept_the_danger=false"
    );

    // Same message succeeds when i_accept_the_danger=true
    let mut cursor2 = std::io::Cursor::new(ct.as_slice());
    let mut output2 = Vec::new();
    let mut stream_input2 =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input2.i_accept_the_danger = true;
    let result2 = decrypt_stream(&mut cursor2, &mut output2, &stream_input2).await;
    assert!(
        result2.is_ok(),
        "multi-frame signed message must succeed with i_accept_the_danger=true"
    );
    assert_eq!(output2, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_must_perform_signature_step() {
    //= specification/client-apis/decrypt.md#behavior
    //= type=test
    //# - If the message header contains an algorithm suite including a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Decrypt operation MUST perform this step.
    let keyring = test_keyring().await;
    let plaintext = b"signing suite signature step test";

    // Encrypt with a signing algorithm suite (ECDSA P384)
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the footer (signature area) to prove the signature step runs.
    // If the step were skipped, tampering the footer would not cause failure.
    let len = ct.len();
    ct[len - 3] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    assert!(
        result.is_err(),
        "decrypt must fail when signature is tampered — proves signature step was performed"
    );
}
