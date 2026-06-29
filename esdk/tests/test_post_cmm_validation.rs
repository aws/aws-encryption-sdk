// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for post-CMM commitment policy validation and decrypt identity KDF.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_post_cmm_commitment_policy_round_trip() {
    // Committing suite with matching policy succeeds; proves post-CMM validation passes.
    let keyring = test_keyring().await;
    let pt = b"test post-cmm commitment policy round trip";
    // Committing suite with RequireEncryptRequireDecrypt: post-CMM validation passes
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;
    let result = decrypt_with(
        &ct,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;
    assert_eq!(
        result.plaintext, pt,
        "committing suite with matching policy must decrypt successfully"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_non_committing_with_require_policy_fails() {
    // Non-committing suite + RequireEncryptRequireDecrypt must fail post-CMM.
    let keyring = test_keyring().await;
    let pt = b"test encrypt non-committing fails";
    // Non-committing suite with RequireEncryptRequireDecrypt: should fail
    let mut enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    let err = result.expect_err(
        "encrypt must fail when algorithm suite is not supported by commitment policy",
    );
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    assert!(
        inner.contains("InvalidAlgorithmSuiteInfoOnEncrypt"),
        "expected InvalidAlgorithmSuiteInfoOnEncrypt, got: {inner}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_non_committing_with_require_policy_fails() {
    // Non-committing suite + RequireEncryptRequireDecrypt must fail on decrypt.
    let keyring = test_keyring().await;
    let pt = b"test decrypt non-committing fails";
    // Encrypt with non-committing suite using ForbidEncryptAllowDecrypt
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    // Decrypt with RequireEncryptRequireDecrypt: should fail because suite is non-committing
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = decrypt(&dec_input).await;
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    let err = result.expect_err(
        "decrypt must fail when algorithm suite is not supported by commitment policy",
    );
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    assert!(
        inner.contains("InvalidAlgorithmSuiteInfoOnDecrypt"),
        "expected InvalidAlgorithmSuiteInfoOnDecrypt, got: {inner}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_identity_kdf_decrypt() {
    // Round-trip with identity KDF suite proves the decrypt path handles non-HKDF derivation.
    let keyring = test_keyring().await;
    let pt = b"test identity kdf on decrypt path";
    // AlgAes256GcmIv12Tag16NoKdf uses identity KDF
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let result = decrypt_with(
        &ct,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    assert_eq!(
        result.plaintext, pt,
        "round-trip with identity KDF suite succeeds"
    );
}
