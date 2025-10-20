// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
use aws_esdk::client::Client as EsdkClient;
use aws_esdk::types::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .keyring(kms_keyring)
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_decrypt_input() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let mut decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());

    decrypt_input.keyring = None;
    let bad_decrypt_output = esdk.decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());

    let cmm = mpl
        .create_default_cryptographic_materials_manager()
        .keyring(kms_keyring.clone())
        .send()
        .await
        .unwrap();
    decrypt_input.keyring = Some(kms_keyring.clone());
    decrypt_input.materials_manager = Some(cmm);
    let bad_decrypt_output = esdk.decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_short() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let cipher_len: usize = esdk_ciphertext.len();
    let mut decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext[..cipher_len])
        .keyring(kms_keyring)
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());

    decrypt_input.ciphertext = &esdk_ciphertext[..cipher_len - 1];
    let bad_decrypt_output = esdk.decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_ec() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encryption_context =
        std::collections::HashMap::from([("stuff".to_string(), "junk".to_string())]);
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .encryption_context(&encryption_context)
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .keyring(kms_keyring)
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_bad_ec() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let encryption_context =
        std::collections::HashMap::from([("aws-crypto-stuff".to_string(), "junk".to_string())]);
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .encryption_context(&encryption_context)
        .keyring(kms_keyring.clone())
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await;

    assert!(encrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_encrypt_input() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let asdf = "asdf";
    let mut encrypt_input = EncryptInputBuilder::default()
        .plaintext(asdf.as_bytes())
        .keyring(kms_keyring.clone())
        .frame_length(0u32)
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await;
    assert!(encrypt_output.is_err());
    encrypt_input.keyring = None;
    encrypt_input.frame_length = None;
    let encrypt_output = esdk.encrypt(&encrypt_input).await;
    assert!(encrypt_output.is_err());

    let cmm = mpl
        .create_default_cryptographic_materials_manager()
        .keyring(kms_keyring.clone())
        .send()
        .await
        .unwrap();
    encrypt_input.keyring = Some(kms_keyring.clone());
    encrypt_input.materials_manager = Some(cmm);
    let encrypt_output = esdk.encrypt(&encrypt_input).await;
    assert!(encrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_single_full_frame() {
    let esdk = EsdkClient::default();
    let mpl = EsdkClient::mpl().unwrap();

    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap();
    let plaintext = "0123456789abcdef";
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext.as_bytes())
        .keyring(kms_keyring.clone())
        .frame_length(plaintext.len() as u32)
        .build()
        .unwrap();
    let encrypt_output = esdk.encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&esdk_ciphertext)
        .keyring(kms_keyring)
        .build()
        .unwrap();
    let decrypt_output = esdk.decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, plaintext.as_bytes());
}
