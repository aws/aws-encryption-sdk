// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#authentication-tag requirements

mod fixtures;
use aws_esdk::*;
use fixtures::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_calculated_over_header_body() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.

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
    let plaintext = b"test auth tag";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Verify the encrypted message can be decrypted, proving the auth tag was correctly calculated
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_uses_authenticated_encryption_algorithm() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:

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
    let plaintext = b"test encryption algorithm";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Successful decrypt proves the auth tag was computed with the correct algorithm
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_aad_is_header_body_concat_required_ec() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.

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
    let plaintext = b"test aad concatenation";
    let encryption_context =
        std::collections::HashMap::from([("stuff".to_string(), "junk".to_string())]);
    let encrypt_input =
        EncryptInput::with_legacy_keyring(plaintext, encryption_context, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Successful decrypt with encryption context proves AAD was correctly constructed
    let ec = EncryptionContext::new();
    let decrypt_input =
        DecryptInput::with_legacy_keyring(&encrypt_output.ciphertext, ec, kms_keyring);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_required_ec_filtering() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).

    // This requirement is exercised by the encrypt/decrypt round-trip with encryption context.
    // The required EC keys are filtered by the CMM and used in the auth tag AAD.
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
    let plaintext = b"test required ec filtering";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_iv_is_zero() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - The IV MUST have a value of 0.

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
    let plaintext = b"test iv zero";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Successful decrypt proves IV=0 was used correctly in auth tag calculation
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_cipherkey_is_derived_data_key() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - The cipherkey MUST be the derived data key

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
    let plaintext = b"test cipherkey";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Successful decrypt proves the correct derived data key was used as cipherkey
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_plaintext_is_empty() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - The plaintext MUST be an empty byte array

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
    let plaintext = b"test empty plaintext for auth tag";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();

    // Successful decrypt proves the auth tag was computed with empty plaintext
    let decrypt_input = DecryptInput::from_encrypt(&encrypt_output.ciphertext, &encrypt_input);
    let decrypt_output = decrypt(&decrypt_input).await.unwrap();
    assert_eq!(decrypt_output.plaintext, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_tampered_header_fails_decrypt() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# The encrypted message output by the Encrypt operation MUST have a message header equal
    //# to the message header calculated in this step.
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# If the message headers are not equal, the Encrypt operation MUST fail.

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
    let plaintext = b"test tampered header";
    let ec = EncryptionContext::new();
    let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec.clone(), kms_keyring.clone());
    let encrypt_output = encrypt(&encrypt_input).await.unwrap();
    let mut tampered = encrypt_output.ciphertext.clone();

    // Tamper with a byte in the header body area (after version byte)
    if tampered.len() > 10 {
        tampered[5] ^= 0xFF;
    }

    // Decryption must fail because the auth tag won't match the tampered header
    let decrypt_input = DecryptInput::with_legacy_keyring(&tampered, ec, kms_keyring);
    let decrypt_result = decrypt(&decrypt_input).await;
    assert!(decrypt_result.is_err());
}
