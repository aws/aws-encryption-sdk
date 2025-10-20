// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
use aws_esdk::client::Client as EsdkClient;
use aws_esdk::types::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encryption_context_on_decrypt() {
    let kms_key = KEY_ARN;
    let asdf = "asdf".as_bytes();

    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();
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

    let encrypt_output = esdk
        .encrypt(
            &EncryptInputBuilder::default()
                .plaintext(asdf)
                .encryption_context(&encryption_context)
                .keyring(kms_keyring.clone())
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    let decrypt_output = esdk
        .decrypt(
            &DecryptInputBuilder::default()
                .ciphertext(&esdk_ciphertext)
                .encryption_context(&encryption_context)
                .keyring(kms_keyring)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(decrypt_output.plaintext == asdf)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encryption_context_on_decrypt_failure() {
    let kms_key = KEY_ARN;
    let asdf = "asdf".as_bytes();

    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();
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

    let encrypt_output = esdk
        .encrypt(
            &EncryptInputBuilder::default()
                .plaintext(asdf)
                .encryption_context(&encryption_context)
                .keyring(kms_keyring.clone())
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    let decrypt_output = esdk
        .decrypt(
            &DecryptInputBuilder::default()
                .ciphertext(&esdk_ciphertext)
                .encryption_context(&bad_encryption_context)
                .keyring(kms_keyring)
                .build()
                .unwrap(),
        )
        .await;

    assert!(decrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]

async fn test_mismatched_encryption_context_on_decrypt() {
    let asdf = "asdf".as_bytes();

    let (namespace, name) = namespace_and_name(0);
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();
    let raw_aes_keyring = mpl
        .create_raw_aes_keyring()
        .key_namespace(namespace)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0; 32]))
        .wrapping_alg(aws_mpl_rs::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();

    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::A);
    let bad_encryption_context =
        small_mismatched_encryption_context(SmallEncryptionContextVariation::A);

    let encrypt_output = esdk
        .encrypt(
            &EncryptInputBuilder::default()
                .plaintext(asdf)
                .encryption_context(&encryption_context)
                .keyring(raw_aes_keyring.clone())
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let esdk_ciphertext = encrypt_output.ciphertext;

    let mut decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .encryption_context(&bad_encryption_context)
        .keyring(raw_aes_keyring.clone())
        .build()
        .unwrap();

    let decrypt_output = esdk.decrypt(&decrypt_input).await;

    // We expect to fail because although the same key is present on the ec
    // their value is different.
    assert!(decrypt_output.is_err());

    decrypt_input.encryption_context = Some(&encryption_context);
    // test that if we supply the right ec we will succeed
    let _ = esdk.decrypt(&decrypt_input).await.unwrap();

    // Since we store all encryption context we MST succeed if no encryption context is
    // supplied on decrypt
    decrypt_input.encryption_context = None;
    let _ = esdk.decrypt(&decrypt_input).await.unwrap();
}
