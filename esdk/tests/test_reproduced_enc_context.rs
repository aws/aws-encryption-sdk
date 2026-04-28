// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#get-the-decryption-materials
//! and specification/client-apis/decrypt.md#input,
//! focusing on the Reproduced Encryption Context requirement.

mod fixtures;
use aws_esdk::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encryption_context_on_decrypt() {
    let kms_key = KEY_ARN;
    let asdf = "asdf".as_bytes();

    let mpl = mpl();
    let supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();

    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();

    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);

    let encrypt_output = encrypt(&EncryptInput::with_legacy_keyring(
        asdf,
        encryption_context.clone(),
        kms_keyring.clone(),
    ))
    .await
    .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    let decrypt_output = decrypt(&DecryptInput::with_legacy_keyring(
        &esdk_ciphertext,
        encryption_context,
        kms_keyring,
    ))
    .await
    .unwrap();

    assert_eq!(
        decrypt_output.plaintext, asdf,
        "successful decrypt proves reproduced encryption context was correctly passed to CMM"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encryption_context_on_decrypt_failure() {
    let kms_key = KEY_ARN;
    let asdf = "asdf".as_bytes();

    let mpl = mpl();
    let supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();

    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();

    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::A);
    let bad_encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);

    let encrypt_output = encrypt(&EncryptInput::with_legacy_keyring(
        asdf,
        encryption_context,
        kms_keyring.clone(),
    ))
    .await
    .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    let decrypt_output = decrypt(&DecryptInput::with_legacy_keyring(
        &esdk_ciphertext,
        bad_encryption_context,
        kms_keyring,
    ))
    .await;

    assert!(
        decrypt_output.is_err(),
        "decrypt must fail when reproduced encryption context is a superset of the original"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_mismatched_encryption_context_on_decrypt() {
    let asdf = "asdf".as_bytes();

    let (namespace, name) = namespace_and_name(0);
    let mpl = mpl();
    let raw_aes_keyring = mpl
        .create_raw_aes_keyring()
        .key_namespace(namespace)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();

    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::A);
    let bad_encryption_context =
        small_mismatched_encryption_context(SmallEncryptionContextVariation::A);

    let encrypt_output = encrypt(&EncryptInput::with_legacy_keyring(
        asdf,
        encryption_context.clone(),
        raw_aes_keyring.clone(),
    ))
    .await
    .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    let mut decrypt_input = DecryptInput::with_legacy_keyring(
        &esdk_ciphertext,
        bad_encryption_context,
        raw_aes_keyring,
    );
    let decrypt_output = decrypt(&decrypt_input).await;

    // We expect to fail because although the same key is present on the ec,
    // the value is different.
    assert!(
        decrypt_output.is_err(),
        "decrypt must fail when reproduced encryption context has mismatched values"
    );

    decrypt_input.encryption_context = encryption_context;
    // Test that if we supply the right ec we will succeed.
    let _ = decrypt(&decrypt_input).await.unwrap();

    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# - Decrypt operation input MUST accept an optional [Encryption Context](#encryption-context) argument.
    decrypt_input.encryption_context = EncryptionContext::new();
    let _ = decrypt(&decrypt_input).await.unwrap();
}
