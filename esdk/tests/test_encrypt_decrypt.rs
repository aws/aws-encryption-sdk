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

    //= specification/client-apis/client.md#encrypt
    //= type=test
    //# The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
    //# that adheres to [encrypt](./encrypt.md).
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let esdk_ciphertext = encrypt_output.ciphertext;

    let decrypt_input = DecryptInput::from_encrypt(&esdk_ciphertext, &encrypt_input);

    //= specification/client-apis/client.md#decrypt
    //= type=test
    //# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
    //# that adheres to [decrypt](./decrypt.md).
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

    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# The Decrypt operation MUST validate that exactly one of a keyring or CMM was provided by the caller.
    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
    let err = bad_decrypt_output.expect_err("decrypt must fail when source = None");
    assert!(
        err.message.contains("materials source") || err.message.to_lowercase().contains("keyring")
            || err.message.to_lowercase().contains("cmm"),
        "error must indicate the missing keyring/CMM, got: {} ({:?})",
        err.message, err.kind
    );
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
    let err = decrypt(&decrypt_input).await.expect_err("truncated ciphertext must fail decrypt");
    // Truncating the last byte corrupts the footer/body; decrypt must report a parse or authentication/serialization failure
    let dbg = format!("{err:?}");
    assert!(
        matches!(
            err.kind,
            aws_esdk::ErrorKind::SerializationError | aws_esdk::ErrorKind::CryptographicError
        ) || dbg.to_lowercase().contains("unexpected")
            || dbg.to_lowercase().contains("length")
            || dbg.to_lowercase().contains("authentic")
            || dbg.to_lowercase().contains("tag"),
        "truncated ciphertext must produce a parse/authentication error, got: {dbg}"
    );
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

    //= specification/client-apis/encrypt.md#encryption-context
    //= type=test
    //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
    //# the encryption operation MUST fail.
    let err = encrypt_output.expect_err("encrypt must fail for aws-crypto- EC key");
    assert!(
        err.message.contains("aws-crypto-") || err.message.to_lowercase().contains("reserved"),
        "error must indicate the reserved-prefix failure, got: {} ({:?})",
        err.message, err.kind
    );
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

    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.
    let err = encrypt_output.expect_err("encrypt must fail when source = None");
    assert!(
        err.message.contains("materials source") || err.message.to_lowercase().contains("keyring")
            || err.message.to_lowercase().contains("cmm"),
        "error must indicate the missing keyring/CMM, got: {} ({:?})",
        err.message, err.kind
    );
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
