// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#behavior

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_skips_signature_step_for_non_signing_algorithm() {
    let keyring = test_keyring().await;
    let plaintext = b"test non-signing decrypt";
    let ec = EncryptionContext::new();

    // Encrypt with a non-signing algorithm suite
    let mut encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    encrypt_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&encrypt_input).await.unwrap().ciphertext;

    // Prove the ciphertext has no footer — so if decrypt tried to run the
    // signature step, it would attempt to read footer bytes from the body
    // region and fail with a parse error.
    assert!(!has_footer(&ct), "non-signing suite must not produce a footer");

    // Decrypt succeeds despite no footer → signature step was skipped.
    let decrypt_input = DecryptInput::from_encrypt(&ct, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();

    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=No footer on wire + decrypt succeeds → signature step was not attempted
    //# - If the message header does not contain an algorithm suite including a signature algorithm,
    //# the Decrypt operation MUST NOT perform this step.
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_output_not_released_until_indicated() {
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
    stream_input.unsafe_release_plaintext_before_verify = true;
    decrypt_stream(&mut cursor, &mut output, &stream_input)
        .await
        .unwrap();
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=Signing-suite streaming decrypt succeeds; output released only after verification
    //# - Output MUST NOT be released until otherwise indicated.
    assert_eq!(output, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_halts_on_incomplete_message() {
    let keyring = test_keyring().await;
    let plaintext = b"truncation test data";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Truncate to ~60% of the message — enough for the header but not the full body
    let truncated = &ct[..ct.len() * 3 / 5];
    let dec_input = DecryptInput::with_legacy_keyring(truncated, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when ciphertext is truncated");
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=Truncated ciphertext triggers SerializationError, proving incomplete input halts
    //# - If all bytes have been provided and this operation
    //# is unable to complete the above steps with the consumable encrypted message bytes,
    //# this operation MUST halt and indicate a failure to the caller.
    assert_eq!(err.kind, ErrorKind::SerializationError, "truncated message must be SerializationError, got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_trailing_bytes_after_message() {
    let keyring = test_keyring().await;
    let plaintext = b"trailing bytes test";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Append extra bytes after the valid message
    ct.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when there are trailing bytes after the message");
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=Appended 4 extra bytes after valid message; decrypt rejects them
    //# - If this operation successfully completes the above steps
    //# but there are consumable bytes which are intended to be decrypted,
    //# this operation MUST fail.
    assert_eq!(err.kind, ErrorKind::Esdk, "trailing bytes must produce Esdk error, got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_fails_for_multi_frame_signed_without_override() {
    let keyring = test_keyring().await;
    // 30 bytes with frame_length=10 → 3 frames (2 regular + 1 final)
    let plaintext = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // decrypt_stream with unsafe_release_plaintext_before_verify=false (default) must fail
    // for multi-frame signed messages
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    // unsafe_release_plaintext_before_verify defaults to false
    assert!(!stream_input.unsafe_release_plaintext_before_verify);
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    let err = result.expect_err("multi-frame signed message must fail with default unsafe_release_plaintext_before_verify=false");
    assert_eq!(err.kind, ErrorKind::Esdk, "multi-frame signed without override must produce Esdk error, got: {err:?}");

    // Same message succeeds when unsafe_release_plaintext_before_verify=true
    let mut cursor2 = std::io::Cursor::new(ct.as_slice());
    let mut output2 = Vec::new();
    let mut stream_input2 =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input2.unsafe_release_plaintext_before_verify = true;
    let result2 = decrypt_stream(&mut cursor2, &mut output2, &stream_input2).await;
    assert!(
        result2.is_ok(),
        "multi-frame signed message must succeed with unsafe_release_plaintext_before_verify=true"
    );
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=Multi-frame signed fails with override=false, succeeds with override=true
    //# - The ESDK MUST provide a configuration option that causes the decryption operation
    //# to fail immediately after parsing the header if a signed algorithm suite is used.
    assert_eq!(output2, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_must_perform_signature_step() {
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
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[len - 3] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when signature is tampered — proves signature step was performed");
    //= spec/client-apis/decrypt.md#behavior
    //= type=test
    //= reason=Tampered footer causes failure, proving signature step runs for signing suites
    //# - If the message header contains an algorithm suite including a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Decrypt operation MUST perform this step.
    assert_eq!(err.kind, ErrorKind::Esdk, "tampered signature must produce Esdk error, got: {err:?}");
}
