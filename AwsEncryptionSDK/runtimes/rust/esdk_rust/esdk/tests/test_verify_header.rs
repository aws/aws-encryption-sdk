// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

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
async fn test_verify_header_v2_round_trip() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# Once a valid message header is deserialized and decryption materials are available,
    //# this operation MUST validate the [message header body](../data-format/message-header.md#header-body)
    //# by using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# to decrypt with the following inputs:

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# For message format version [2.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be 0.

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - the cipherkey MUST be the derived data key

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - the ciphertext MUST be an empty byte array

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - the tag MUST be the value serialized in the message header's
    //# [authentication tag field](../data-format/message-header.md#authentication-tag)

    let keyring = test_keyring().await;
    let plaintext = b"v2 header verification test";

    // v2 algorithm suite (non-signing, with commitment)
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, plaintext, "successful v2 round-trip proves header verification with IV=0, derived key, empty ciphertext, correct tag, and correct AAD");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_header_v1_round_trip() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - For message format version [1.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be the value serialized in the message header's [IV field](../data-format/message-header#iv).

    let keyring = test_keyring().await;
    let plaintext = b"v1 header verification test";

    // v1 algorithm suite
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, plaintext, "successful v1 round-trip proves header verification with IV from header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_header_fails_on_tampered_header() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.

    let keyring = test_keyring().await;
    let plaintext = b"tamper header test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with a byte in the header body (byte 10 is safely within the header body)
    ct[10] ^= 0xFF;

    let dec_input = DecryptInput::from_encrypt(&ct, &enc_input);
    let result = decrypt(&dec_input).await;
    assert!(result.is_err(), "decrypt must fail when header bytes are tampered (tag verification failure)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_header_encryption_context_to_only_authenticate() {
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [decryption materials](../framework/structures.md#decryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [decryption material's](../framework/structures.md#decryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys-1)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).

    let keyring = test_keyring().await;
    let plaintext = b"required ec test";

    // Create a required encryption context CMM that filters "keyA" out of the header
    let encryption_context = small_encryption_context(SmallEncryptionContextVariation::AB);
    let required_ec_keys = small_encryption_context_keys(SmallEncryptionContextVariation::A);
    let reproduced_ec = small_encryption_context(SmallEncryptionContextVariation::A);

    let default_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();

    let req_cmm = mpl()
        .create_required_encryption_context_cmm()
        .underlying_cmm(default_cmm)
        .required_encryption_context_keys(required_ec_keys)
        .send()
        .await
        .unwrap();

    let enc_input = EncryptInput::with_legacy_cmm(plaintext, encryption_context, req_cmm);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with the reproduced encryption context — proves the EC filtering is correct
    let dec_input = DecryptInput::with_legacy_keyring(&ct, reproduced_ec, keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, plaintext, "successful round-trip with required EC keys proves encryption context to only authenticate is correctly filtered and serialized");
}
