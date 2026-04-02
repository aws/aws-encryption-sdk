// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for keyring-to-default-CMM requirements:
//! - specification/client-apis/decrypt.md#keyring
//! - specification/client-apis/encrypt.md#get-the-encryption-materials
//!
//! Note: The modern `MaterialSource::Keyring` path is not yet testable because
//! `create_raw_aes_keyring` and `create_default_cryptographic_materials_manager`
//! are not implemented in the modern MPL. These tests use the legacy keyring path
//! (`MaterialSource::LegacyKeyring`) which exercises the same conceptual behavior:
//! constructing a default CMM from a keyring and using it to obtain materials.

mod fixtures;

use aws_esdk::*;
use fixtures::*;

/// Create a legacy raw AES keyring for testing (no KMS needed).
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

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_decrypt() {
    //= specification/client-apis/decrypt.md#keyring
    //= type=test
    //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
    //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
    let keyring = test_keyring().await;
    let pt = b"test keyring constructs default cmm for decrypt";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_default_cmm_obtains_decryption_materials() {
    //= specification/client-apis/decrypt.md#keyring
    //= type=test
    //# This default CMM constructed from the keyring MUST obtain the decryption materials required for decryption.
    let keyring = test_keyring().await;
    let pt = b"test default cmm obtains decryption materials";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_encrypt() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
    //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
    //# constructed using the caller-supplied keyring as input.
    let keyring = test_keyring().await;
    let pt = b"test keyring constructs default cmm for encrypt";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    let result = encrypt(&enc_input).await;
    assert!(result.is_ok(), "encrypt with keyring must succeed via default CMM");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
    //= specification/client-apis/decrypt.md#keyring
    //= type=test
    //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
    //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
    let keyring = test_keyring().await;
    let pt = b"negative test keyring to default cmm";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with a different keyring (different key material) — should fail
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
    assert!(
        result.is_err(),
        "decrypt must fail when default CMM cannot obtain decryption materials with wrong keyring"
    );
}
