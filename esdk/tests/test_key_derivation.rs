// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/encrypt.md#get-the-encryption-materials
//! Key derivation requirements.

mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

/// Encrypt then decrypt with a specific algorithm suite and commitment policy.
async fn round_trip_with_suite(
    plaintext: &[u8],
    suite: EsdkAlgorithmSuiteId,
    policy: EsdkCommitmentPolicy,
) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    enc_input.commitment_policy = policy;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = policy;
    decrypt(&dec_input).await.unwrap().plaintext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_derivation_uses_suite_kdf() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    let pt = b"test kdf selection from suite";
    let result = round_trip_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
    )
    .await;
    assert_eq!(
        result, pt,
        "round-trip proves the correct KDF from the algorithm suite was used"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_identity_kdf_derived_key_equals_plaintext_key() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
    //# then the derived data key MUST be the same as the plaintext data key.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
    //# then the derived data key MUST be the same as the plaintext data key.
    let pt = b"test identity kdf";
    let result = round_trip_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
    )
    .await;
    assert_eq!(
        result, pt,
        "round-trip with identity KDF proves derived key equals plaintext key"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_hkdf_derivation_process() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
    //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
    let pt = b"test hkdf derivation";
    let result = round_trip_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
    )
    .await;
    assert_eq!(
        result, pt,
        "round-trip with HKDF suite proves correct HKDF derivation process"
    );
}
