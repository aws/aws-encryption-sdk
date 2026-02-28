// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
use aws_esdk::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    let mpl = mpl();

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
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(asdf.as_bytes(), ec, kms_keyring);
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;

    let decrypt_input = DecryptInput::from_encrypt(&esdk_ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_decrypt_input() {
    let mpl = mpl();

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
    let ec = EncryptionContext::new();
    let encrypt_input =
        EncryptInput::with_legacy_keyring(asdf.as_bytes(), ec.clone(), kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let mut decrypt_input =
        DecryptInput::with_legacy_keyring(&esdk_ciphertext, ec, kms_keyring.clone());
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());

    decrypt_input.source = None;
    let bad_decrypt_output = decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_short() {
    let mpl = mpl();

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
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(asdf.as_bytes(), ec, kms_keyring);
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let cipher_len: usize = esdk_ciphertext.len();
    let mut decrypt_input =
        DecryptInput::from_encrypt(&esdk_ciphertext[..cipher_len], &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, asdf.as_bytes());

    decrypt_input.ciphertext = &esdk_ciphertext[..cipher_len - 1];
    let bad_decrypt_output = decrypt(&decrypt_input).await;
    assert!(bad_decrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_ec() {
    let mpl = mpl();

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
    let asdf = "asdf".as_bytes();
    let encryption_context =
        std::collections::HashMap::from([("stuff".to_string(), "junk".to_string())]);
    let encrypt_input =
        EncryptInput::with_legacy_keyring(asdf, encryption_context, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;
    let ec = EncryptionContext::new();
    let decrypt_input = DecryptInput::with_legacy_keyring(&esdk_ciphertext, ec, kms_keyring);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();

    assert_eq!(decrypt_output.plaintext, asdf);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_bad_ec() {
    let mpl = mpl();

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
    let asdf = "asdf".as_bytes();
    let encryption_context =
        std::collections::HashMap::from([("aws-crypto-stuff".to_string(), "junk".to_string())]);
    let encrypt_input = EncryptInput::with_legacy_keyring(asdf, encryption_context, kms_keyring);
    let encrypt_output = encrypt(&encrypt_input).await;

    assert!(encrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_encrypt_input() {
    let mpl = mpl();

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
    let asdf = "asdf".as_bytes();
    let ec = EncryptionContext::new();
    let mut encrypt_input = EncryptInput::with_legacy_keyring(asdf, ec, kms_keyring.clone());
    encrypt_input.source = None;
    let encrypt_output = encrypt(&encrypt_input).await;
    assert!(encrypt_output.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_single_full_frame() {
    let mpl = mpl();

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
    let plaintext = "0123456789abcdef".as_bytes();

    let ec = EncryptionContext::new();
    let mut encrypt_input =
        EncryptInput::with_legacy_keyring(plaintext, ec.clone(), kms_keyring.clone());
    for i in 4..=plaintext.len() {
        encrypt_input.frame_length.0 = std::num::NonZeroU32::new(i as u32).unwrap();
        let encrypt_output = encrypt(&encrypt_input).await.unwrap();
        let esdk_ciphertext = encrypt_output.ciphertext;
        let decrypt_input =
            DecryptInput::with_legacy_keyring(&esdk_ciphertext, ec.clone(), kms_keyring.clone());
        let decrypt_output = decrypt(&decrypt_input).await.unwrap();
        assert_eq!(decrypt_output.plaintext, plaintext);
    }
}
