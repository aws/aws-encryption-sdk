// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for post-CMM commitment policy validation and decrypt identity KDF.

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

/// Decrypt with a specific policy, return plaintext.
async fn decrypt_with(
    ciphertext: &[u8],
    policy: EsdkCommitmentPolicy,
    keyring: &aws_mpl_legacy::dafny::types::keyring::KeyringRef,
) -> Vec<u8> {
    let mut dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring.clone());
    dec_input.commitment_policy = policy;
    decrypt(&dec_input).await.unwrap().plaintext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_post_cmm_commitment_policy_encrypt() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    let keyring = test_keyring().await;
    let pt = b"test post-cmm commitment policy on encrypt";
    // Committing suite with RequireEncryptRequireDecrypt: post-CMM validation passes
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;
    let result = decrypt_with(&ct, EsdkCommitmentPolicy::RequireEncryptRequireDecrypt, &keyring).await;
    assert_eq!(result, pt, "round-trip proves post-CMM commitment policy validation passed on encrypt");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_post_cmm_commitment_policy_decrypt() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    let keyring = test_keyring().await;
    let pt = b"test post-cmm commitment policy on decrypt";
    // Committing suite with RequireEncryptRequireDecrypt: post-CMM validation passes
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;
    let result = decrypt_with(&ct, EsdkCommitmentPolicy::RequireEncryptRequireDecrypt, &keyring).await;
    assert_eq!(result, pt, "round-trip proves post-CMM commitment policy validation passed on decrypt");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_non_committing_with_require_policy_fails() {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    let keyring = test_keyring().await;
    let pt = b"test encrypt non-committing fails";
    // Non-committing suite with RequireEncryptRequireDecrypt: should fail
    let mut enc_input =
        EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    assert!(result.is_err(), "encrypt must fail when algorithm suite is not supported by commitment policy");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_non_committing_with_require_policy_fails() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    let keyring = test_keyring().await;
    let pt = b"test decrypt non-committing fails";
    // Encrypt with non-committing suite using ForbidEncryptAllowDecrypt
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    // Decrypt with RequireEncryptRequireDecrypt: should fail because suite is non-committing
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "decrypt must fail when algorithm suite is not supported by commitment policy");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_identity_kdf_decrypt() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
    //# then the derived data key MUST be the same as the plaintext data key.
    let keyring = test_keyring().await;
    let pt = b"test identity kdf on decrypt path";
    // AlgAes256GcmIv12Tag16NoKdf uses identity KDF
    let ct = encrypt_with(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let result = decrypt_with(&ct, EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt, &keyring).await;
    assert_eq!(result, pt, "round-trip with identity KDF proves derived key equals plaintext key on decrypt");
}
