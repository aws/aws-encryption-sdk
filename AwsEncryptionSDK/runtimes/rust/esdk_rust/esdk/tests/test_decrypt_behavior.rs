// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/client-apis/decrypt.md#behavior

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
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#behavior
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
