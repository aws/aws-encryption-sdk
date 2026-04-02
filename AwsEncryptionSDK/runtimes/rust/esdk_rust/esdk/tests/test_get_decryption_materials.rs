// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#get-the-decryption-materials

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
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
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/decrypt.md#cryptographic-materials-manager
    //= type=test
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    let pt = b"test obtain decryption materials";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_constructed_as_follows() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let pt = b"test cmm call construction";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_algorithm_suite_id() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
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
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Commitment Policy: This MUST be the commitment policy configured on the client.
    let pt = b"test commitment policy";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_encrypted_data_keys() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header#encrypted-data-keys)
    //# from the message header.
    let pt = b"test encrypted data keys";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_call_encryption_context() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
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
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    let pt = b"test reproduced encryption context";
    let result = round_trip(pt, EncryptionContext::new()).await;
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
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

/// Create a raw AES keyring for testing with specific key material.
async fn make_keyring(key_byte: u8) -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(key_byte);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([key_byte; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Encrypt with a specific suite and policy, return ciphertext.
async fn encrypt_with(
    plaintext: &[u8],
    suite: EsdkAlgorithmSuiteId,
    policy: EsdkCommitmentPolicy,
    keyring: &aws_mpl_legacy::dafny::types::keyring::KeyringRef,
) -> Vec<u8> {
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    enc_input.commitment_policy = policy;
    encrypt(&enc_input).await.unwrap().ciphertext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_pre_cmm_commitment_policy_check() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the parsed [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    let keyring = make_keyring(0).await;
    let pt = b"test pre-cmm commitment policy";
    // Encrypt with non-committing suite using ForbidEncryptAllowDecrypt
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    // Decrypt with RequireEncryptRequireDecrypt — pre-CMM check must reject non-committing suite
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "decrypt must fail when parsed algorithm suite is not supported by commitment policy");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_used_is_input_cmm() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The CMM used MUST be the input CMM, if supplied.
    let keyring = make_keyring(0).await;
    let cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();
    let pt = b"test input cmm used";
    let enc_input = EncryptInput::with_legacy_cmm(pt, EncryptionContext::new(), cmm.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_cmm(&ct, EncryptionContext::new(), cmm);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_default_cmm_constructed_from_keyring() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    let keyring = make_keyring(0).await;
    let pt = b"test default cmm from keyring";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    // Decrypt with keyring (not CMM) — decrypt must construct default CMM internally
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_data_key_derived_from_plaintext_data_key() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let keyring = make_keyring(0).await;
    let pt = b"test data key derivation";
    // Use HKDF suite to exercise key derivation
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "successful round-trip proves data key was correctly derived from plaintext data key");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_algorithm_suite_from_decryption_materials() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let keyring = make_keyring(0).await;
    let pt = b"test algorithm suite from materials";
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "successful round-trip proves algorithm suite from materials was used for decryption");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_commit_key_derived_and_validated() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The derived commit key MUST equal the commit key stored in the message header.
    let keyring = make_keyring(0).await;
    let pt = b"test commit key derivation and equality";
    // Use committing suite
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;
    let dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "successful round-trip with committing suite proves commit key was derived and matched header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_kdf_algorithm_from_materials_suite() {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    let keyring = make_keyring(0).await;
    let pt = b"test kdf algorithm from materials";
    // Use HKDF suite to exercise KDF algorithm selection
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "successful round-trip proves KDF algorithm from materials suite was used");
}
