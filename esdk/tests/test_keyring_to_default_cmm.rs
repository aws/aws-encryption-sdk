// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for keyring-to-default-CMM requirements:
//! - spec/client-apis/decrypt.md#keyring
//! - spec/client-apis/encrypt.md#get-the-encryption-materials
//!
//! Note: The modern `MaterialSource::Keyring` path is not yet testable because
//! `create_raw_aes_keyring` and `create_default_cryptographic_materials_manager`
//! are not implemented in the modern MPL. These tests use the legacy keyring path
//! (`MaterialSource::LegacyKeyring`) which exercises the same conceptual behavior:
//! constructing a default CMM from a keyring and using it to obtain materials.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_decrypt() {
    let keyring = test_keyring().await;
    let pt = b"test keyring constructs default cmm for decrypt";
    let mut ec = EncryptionContext::new();
    ec.insert("purpose".to_string(), "keyring-cmm-test".to_string());
    let enc_input =
        EncryptInput::with_legacy_keyring(pt, ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, ec, keyring);
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#keyring
    //= type=test
    //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
    //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
    //
    //= spec/client-apis/decrypt.md#keyring
    //= type=test
    //# This default CMM constructed from the keyring MUST obtain the decryption materials required for decryption.
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_encrypt() {
    let keyring = test_keyring().await;
    let pt = b"test keyring constructs default cmm for encrypt";
    let enc_input =
        EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
    //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
    //# constructed using the caller-supplied keyring as input.
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
    let keyring = test_keyring().await;
    let pt = b"negative test keyring to default cmm";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with a different keyring (different key material) — should fail
    let wrong_keyring = aes_keyring(1).await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), wrong_keyring);
    let result = decrypt(&dec_input).await;
    //= spec/client-apis/decrypt.md#keyring
    //= type=test
    //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
    //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
    assert!(
        result.is_err(),
        "decrypt must fail when default CMM cannot obtain decryption materials with wrong keyring"
    );
}
