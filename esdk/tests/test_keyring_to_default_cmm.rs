// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for keyring-to-default-CMM requirements:
//! - spec/client-apis/decrypt.md#keyring
//! - spec/client-apis/encrypt.md#get-the-encryption-materials
//!
//! Strategy: a black-box round-trip with a keyring proves nothing about *which*
//! CMM was constructed — any working CMM that wraps the keyring would pass. To
//! actually prove "the keyring path constructs a default CMM (not some other
//! CMM)," each test below exercises *cross-compatibility*: encrypt under one
//! path (e.g. an explicit default CMM constructed independently) and decrypt
//! under the other (the keyring → default-CMM-internal path). If the keyring
//! path constructed any CMM other than the default-CMM-of-K, the materials
//! would diverge and the cross-decrypt would fail.
//!
//! This is the strongest property we can assert without a spy on the CMM
//! construction in `materials.rs` — the legacy MPL doesn't expose a way to ask
//! a `CryptographicMaterialsManagerRef` "are you the default CMM?" at runtime.
//!
//! Note: The modern `MaterialSource::Keyring` path is not yet testable because
//! `create_raw_aes_keyring` and `create_default_cryptographic_materials_manager`
//! are not implemented in the modern MPL. These tests use the legacy keyring
//! path (`MaterialSource::LegacyKeyring`).

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

/// Helper: independently construct a default CMM from a keyring via the MPL.
/// Mirrors the call in `materials.rs::create_cmm_from_input` for the
/// `MaterialSource::LegacyKeyring` arm.
async fn default_cmm_from(
    keyring: aws_mpl_legacy::dafny::types::keyring::KeyringRef,
) -> aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef
{
    mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring)
        .send()
        .await
        .expect("default CMM construction must succeed for the test keyring")
}

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_decrypt() {
    // Cross-compat: encrypt with an *explicit* default CMM built from K, then
    // decrypt with K alone. If decrypt's keyring path constructed any CMM
    // other than `create_default_cryptographic_materials_manager(K)`, the
    // decryption materials would diverge and decrypt would fail.
    let keyring = test_keyring().await;
    let pt = b"keyring constructs default cmm for decrypt";
    let mut ec = EncryptionContext::new();
    ec.insert("purpose".to_string(), "keyring-cmm-test".to_string());

    let default_cmm = default_cmm_from(keyring.clone()).await;
    let enc_input = EncryptInput::with_legacy_cmm(pt, ec.clone(), default_cmm);
    let ct = encrypt(&enc_input)
        .await
        .expect("encrypt with explicit default CMM must succeed");

    let dec_input = DecryptInput::with_legacy_keyring(&ct.ciphertext, ec, keyring);
    //= spec/client-apis/decrypt.md#keyring
    //= type=test
    //= reason=Ciphertext encrypted under an explicit default CMM decrypts under the keyring path; a different CMM would diverge on materials
    //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
    //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
    //
    //= spec/client-apis/decrypt.md#keyring
    //= type=test
    //= reason=The keyring path obtains decryption materials for the explicit-default-CMM ciphertext, proving the materials path matches
    //# This default CMM constructed from the keyring MUST obtain the decryption materials required for decryption.
    let result = decrypt(&dec_input)
        .await
        .expect("decrypt with keyring must succeed on explicit-default-CMM ciphertext");
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_keyring_constructs_default_cmm_for_encrypt() {
    // Cross-compat (other direction): encrypt with K alone (exercising the
    // encrypt-side keyring → default-CMM path), then decrypt with an
    // *explicit* default CMM built from K. If encrypt's keyring path
    // constructed any CMM other than the default-CMM-of-K, the explicit
    // default CMM would not be able to decrypt the message.
    let keyring = test_keyring().await;
    let pt = b"keyring constructs default cmm for encrypt";
    let ec = EncryptionContext::new();

    let enc_input = EncryptInput::with_legacy_keyring(pt, ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input)
        .await
        .expect("encrypt with keyring must succeed");

    let default_cmm = default_cmm_from(keyring).await;
    let dec_input = DecryptInput::with_legacy_cmm(&ct.ciphertext, ec, default_cmm);
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Ciphertext produced via the keyring path decrypts under an explicit default CMM, proving the encrypt-side path constructs a default CMM
    //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
    //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
    //# constructed using the caller-supplied keyring as input.
    let result = decrypt(&dec_input)
        .await
        .expect("decrypt with explicit default CMM must succeed on keyring-encrypted ciphertext");
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
    // Negative: a keyring whose key material differs from the encrypt-side
    // keyring cannot unwrap any EDK in the message. Exercises the failure
    // mode of the keyring → default-CMM path.
    let keyring = test_keyring().await;
    let pt = b"negative test keyring to default cmm";
    let enc_input = EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let wrong_keyring = aes_keyring(1).await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), wrong_keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err(
        "decrypt must fail when default CMM cannot obtain decryption materials with wrong keyring",
    );
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    assert!(
        inner.contains("Raw AES Keyring was unable to decrypt"),
        "expected raw-AES decrypt failure, got: {inner}"
    );
}
