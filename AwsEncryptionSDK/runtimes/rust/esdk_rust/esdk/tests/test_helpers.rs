// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

mod fixtures;

use aws_esdk::*;
use fixtures::*;

pub const IV_LEN: usize = 12;
pub const TAG_LEN: usize = 16;
pub const ENDFRAME_MARKER: [u8; 4] = 0xFFFF_FFFFu32.to_be_bytes();

#[derive(Clone, Copy, Debug)]
pub enum Version {
    V1,
    V2,
}

pub const VERSIONS: [Version; 2] = [Version::V1, Version::V2];

use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::dafny::types::keyring::KeyringRef;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;

/// Create a raw AES keyring for testing (no KMS needed).
pub async fn test_keyring() -> KeyringRef {
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

// ── Keyring helpers ──

/// Create a raw AES keyring with key material derived from `n`.
pub async fn aes_keyring(n: u8) -> KeyringRef {
    let (ns, name) = namespace_and_name(n);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([n; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Create a multi-keyring from a generator and child keyrings.
pub async fn multi_keyring(
    generator_kr: KeyringRef,
    children: Vec<KeyringRef>,
) -> KeyringRef {
    mpl()
        .create_multi_keyring()
        .generator(generator_kr)
        .child_keyrings(children)
        .send()
        .await
        .unwrap()
}

// ── Encrypt helpers ──

/// Encrypt with defaults, return full EncryptOutput.
pub async fn encrypt_default(plaintext: &[u8]) -> EncryptOutput {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap()
}

/// Encrypt with a V1 (non-committing) algorithm suite, return ciphertext bytes.
pub async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a V1 (non-committing) algorithm suite and encryption context, return ciphertext bytes.
pub async fn encrypt_v1_with_ec(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with default (V2) algorithm suite, return ciphertext bytes.
pub async fn encrypt_v2(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt plaintext with a given frame length, return ciphertext bytes.
pub async fn encrypt_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_with_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a non-signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_without_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a specific suite, policy, and keyring, return ciphertext bytes.
pub async fn encrypt_with_suite(
    plaintext: &[u8],
    suite: EsdkAlgorithmSuiteId,
    policy: EsdkCommitmentPolicy,
    keyring: &KeyringRef,
) -> Vec<u8> {
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    enc_input.commitment_policy = policy;
    encrypt(&enc_input).await.unwrap().ciphertext
}

/// Encrypt with a specific version and keyring, return ciphertext bytes.
pub async fn encrypt_with_version(
    plaintext: &[u8],
    version: Version,
    keyring: KeyringRef,
) -> Vec<u8> {
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    if let Version::V1 = version {
        input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
        input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    }
    encrypt(&input).await.unwrap().ciphertext
}

// ── Decrypt helpers ──

/// Decrypt ciphertext with the default test keyring, return full DecryptOutput.
pub async fn decrypt_ciphertext(ciphertext: &[u8]) -> DecryptOutput {
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap()
}

/// Decrypt with a specific policy and keyring, return full DecryptOutput.
pub async fn decrypt_with(
    ciphertext: &[u8],
    policy: EsdkCommitmentPolicy,
    keyring: &KeyringRef,
) -> DecryptOutput {
    let mut dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring.clone());
    dec_input.commitment_policy = policy;
    decrypt(&dec_input).await.unwrap()
}

// ── Round-trip helpers ──

/// Encrypt then decrypt round-trip, return decrypted plaintext.
pub async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with a given frame length, return decrypted plaintext.
pub async fn round_trip_framed(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt V1 then decrypt, returning decrypted plaintext.
pub async fn round_trip_v1(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let ct = encrypt_v1_with_ec(plaintext, ec).await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt V2 then decrypt, returning decrypted plaintext.
pub async fn round_trip_v2(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    let ct = encrypt(&input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with a signing algorithm suite.
pub async fn round_trip_signing(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with encryption context, returning full DecryptOutput.
pub async fn round_trip_with_ec(plaintext: &[u8], ec: EncryptionContext) -> DecryptOutput {
    let keyring = test_keyring().await;
    let enc_input = EncryptInput::with_legacy_keyring(plaintext, ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, ec, keyring);
    decrypt(&dec_input).await.unwrap()
}
