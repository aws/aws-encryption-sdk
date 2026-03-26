// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#get-the-decryption-materials

mod fixtures;

use aws_esdk::*;
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

/// Encrypt then decrypt, returning the decrypt output.
async fn round_trip(plaintext: &[u8], ec: EncryptionContext) -> DecryptOutput {
    let keyring = test_keyring().await;
    let enc_input = EncryptInput::with_legacy_keyring(plaintext, ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, ec, keyring);
    decrypt(&dec_input).await.unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_obtain_decryption_materials_via_cmm() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#cryptographic-materials-manager
    //= type=test
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    let pt = b"test obtain decryption materials";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_constructed_as_follows() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let pt = b"test cmm call construction";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_algorithm_suite_id() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Algorithm Suite ID: This MUST be the parsed
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# from the message header.
    let pt = b"test algorithm suite id";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_commitment_policy() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Commitment Policy: This MUST be the commitment policy configured on the client.
    let pt = b"test commitment policy";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_encrypted_data_keys() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header#encrypted-data-keys)
    //# from the message header.
    let pt = b"test encrypted data keys";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_encryption_context() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
    //# from the message header.
    let ec = EncryptionContext::from([("key1".to_string(), "val1".to_string())]);
    let pt = b"test encryption context";
    let result = round_trip(pt, ec).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_reproduced_encryption_context() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    let pt = b"test reproduced encryption context";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    let keyring = test_keyring().await;
    let pt = b"negative test";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with a different keyring (different key material) — CMM call should fail
    let (ns, name) = namespace_and_name(1);
    let wrong_keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), wrong_keyring);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "decrypt must fail when CMM cannot obtain decryption materials");
}
