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
    //
    //= spec/client-apis/decrypt.md#algorithm-suite
    //= type=test
    //= reason=Output suite matches a supported ESDK suite (AlgAes256GcmHkdfSha512CommitKey)
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    assert_eq!(result.algorithm_suite_id, suite);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_no_unauthenticated_data_released() {
    let keyring = test_keyring().await;
    let plaintext = b"tamper test data";
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the ciphertext (flip a byte near the end, in the signature
    // area for the default signing suite).
    let mut tampered_ct = valid_ct.clone();
    let tamper_pos = tampered_ct.len() - 50;
    tampered_ct[tamper_pos] ^= 0xFF;

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#authenticated-data
    //= type=test
    //= reason=Untampered ct decrypts (Ok with plaintext); tampered ct → Err with no DecryptOutput, no plaintext released
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    assert!(decrypt(&valid_input).await.is_ok(), "valid ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::Esdk,
        "tampered ct must produce Esdk error, no plaintext"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_callers_must_discard_on_failure() {
    let keyring = test_keyring().await;
    let plaintext = b"discard on failure test";
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the last few bytes (signature area) to cause verification failure.
    let mut tampered_ct = valid_ct.clone();
    let len = tampered_ct.len();
    tampered_ct[len - 5] ^= 0xFF;

    let mut valid_cursor = std::io::Cursor::new(valid_ct.as_slice());
    let mut valid_output = Vec::new();
    let mut tampered_cursor = std::io::Cursor::new(tampered_ct.as_slice());
    let mut tampered_output = Vec::new();
    let mut stream_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    stream_input.unsafe_release_plaintext_before_verify = true;

    //= spec/client-apis/decrypt.md#security-considerations
    //= type=test
    //= reason=Untampered ct streams to Ok; tampered signature → Err signals callers to discard any released output
    //# Additionally, if this operation fails, callers MUST discard the released plaintext and encryption context
    //# and MUST rollback any processing done due to the released plaintext or encryption context.
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
