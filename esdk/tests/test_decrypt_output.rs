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
    // Non-signing suite so the per-frame auth tag is the integrity layer under test:
    // tampering it makes the body plaintext unauthenticated (on-point for this requirement,
    // unlike tampering the post-authentication signature).
    let keyring = test_keyring().await;
    let pt = vec![0xABu8; 20];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&pt, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    enc_input.frame_length = FrameLength::new(10).unwrap();
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper the first regular frame's auth tag: [seq_num(4)][IV(12)][content(10)][tag(16)].
    let body_start = find_body_start(&valid_ct, 10).expect("must find body");
    let tag_end = body_start + 4 + IV_LEN + 10 + TAG_LEN - 1;
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[tag_end] ^= 0xFF;

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#authenticated-data
    //= type=test
    //= reason=Untampered ct decrypts; tampered body auth tag → CryptographicError with no DecryptOutput, so unauthenticated plaintext is never released
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    assert!(decrypt(&valid_input).await.is_ok(), "valid ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::CryptographicError,
        "tampered body auth tag must produce CryptographicError, no plaintext"
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
    //= reason=SDK can only signal failure; tampered signature → Err is the discard trigger, the caller-side discard/rollback is not SDK-observable
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
