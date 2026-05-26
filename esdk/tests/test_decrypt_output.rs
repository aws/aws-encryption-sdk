// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for decrypt.md sections: #output, #algorithm-suite, #authenticated-data, #security-considerations

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_includes_encryption_context() {
    let keyring = test_keyring().await;
    let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
    let enc_input = EncryptInput::with_legacy_keyring(b"ec test", ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    for (k, v) in &ec {
    //= spec/client-apis/decrypt.md#output
    //= type=test
    //# - Decrypt operation output MUST include an [encryption context](#encryption-context) value.
        assert_eq!(result.encryption_context.get(k).unwrap(), v);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_output_includes_algorithm_suite() {
    let keyring = test_keyring().await;
    let suite = EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#output
    //= type=test
    //# - Decrypt operation output MUST include an [algorithm suite](#algorithm-suite) value.
    assert_eq!(result.algorithm_suite_id, suite);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_data_released() {
    let keyring = test_keyring().await;
    let plaintext = b"tamper test data";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the ciphertext body (flip a byte near the end, in the encrypted content area)
    let tamper_pos = ct.len() - 50;
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[tamper_pos] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when ciphertext is tampered — no unauthenticated data released");
    //= spec/client-apis/decrypt.md#authenticated-data
    //= type=test
    //= reason=Tampered body produces error with no plaintext returned to caller
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_callers_must_discard_on_failure() {
    let keyring = test_keyring().await;
    let plaintext = b"discard on failure test";
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the last few bytes (signature area) to cause verification failure
    let len = ct.len();
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[len - 5] ^= 0xFF;

    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.unsafe_release_plaintext_before_verify = true;
    let result = decrypt_stream(&mut cursor, &mut output, &stream_input).await;
    let err = result.expect_err("decrypt_stream must return Err on tampered signature — callers must discard output");
    //= spec/client-apis/decrypt.md#security-considerations
    //= type=test
    //# Additionally, if this operation fails, callers MUST discard the released plaintext and encryption context
    //# and MUST rollback any processing done due to the released plaintext or encryption context.
    //= reason=Tampered signature → Err signals callers to discard released output
    assert_eq!(err.kind, ErrorKind::Esdk, "got: {err:?}");
}
