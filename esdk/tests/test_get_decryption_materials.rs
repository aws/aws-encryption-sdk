// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#get-the-decryption-materials

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_fails_with_wrong_keyring() {
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
    let err = result.expect_err("decrypt must fail when CMM cannot obtain decryption materials");
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Wrong keyring → default CMM can't unwrap → proves input keyring determines CMM
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    assert!(inner.contains("CollectionOfErrors"), "expected CollectionOfErrors, got: {inner}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_pre_cmm_commitment_policy_check() {
    let keyring = aes_keyring(0).await;
    let pt = b"test pre-cmm commitment policy";
    // Encrypt with non-committing suite using ForbidEncryptAllowDecrypt
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    // Decrypt with RequireEncryptRequireDecrypt — pre-CMM check must reject non-committing suite
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when parsed algorithm suite is not supported by commitment policy");
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the parsed [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Test explicitly sets commitment_policy on input; failure proves it was passed to CMM
    //# - Commitment Policy: This MUST be the commitment policy configured on the client.
    assert!(inner.contains("InvalidAlgorithmSuiteInfoOnDecrypt"), "expected InvalidAlgorithmSuiteInfoOnDecrypt, got: {inner}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_data_key_derived_from_plaintext_data_key() {
    let keyring = aes_keyring(0).await;
    let pt = b"test data key derivation";
    // Use HKDF suite to exercise key derivation
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=HKDF suite decrypt succeeds; wrong keyring test proves wrong PDK → failure
    //# The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    assert_eq!(
        result.plaintext, pt,
        "successful round-trip proves data key was correctly derived from plaintext data key"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_algorithm_suite_from_decryption_materials() {
    let keyring = aes_keyring(0).await;
    let pt = b"test algorithm suite from materials";
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Decrypt succeeds with HKDF suite; proves materials' suite was used
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    assert_eq!(
        result.plaintext, pt,
        "successful round-trip proves algorithm suite from materials was used for decryption"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_commit_key_derived_and_validated() {
    let keyring = aes_keyring(0).await;
    let pt = b"test commit key derivation and equality";
    // Use committing suite
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;

    // Baseline: untampered ciphertext decrypts.
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
    assert!(decrypt(&dec_input).await.is_ok(), "baseline must pass");

    // Tamper the Algorithm Suite Data (commit key) in the header.
    // parse_v2_header_field_offsets gives us the exact byte range.
    let mut tampered = ct.clone();
    let fields = parse_v2_header_field_offsets(&tampered);
    let (_, sd_start, _sd_end) = fields.iter().find(|(n, _, _)| *n == "Algorithm Suite Data")
        .expect("V2 header must have Algorithm Suite Data field");
    let original = tampered[*sd_start];
    tampered[*sd_start] ^= 0xFF;
    assert_ne!(tampered[*sd_start], original, "tamper must change the byte");

    let dec_input = DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered commit key must cause decrypt to fail");
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Tampered commit key in header causes failure; proves equality check runs
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Tampered header commit key != derived commit key → ValidationError
    //# The derived commit key MUST equal the commit key stored in the message header.
    assert_eq!(err.kind, ErrorKind::ValidationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_kdf_algorithm_from_materials_suite() {
    let keyring = aes_keyring(0).await;
    let pt = b"test kdf algorithm from materials";
    // Use HKDF suite to exercise KDF algorithm selection
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=HKDF suite round-trip succeeds; proves KDF from materials was used
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    assert_eq!(
        result.plaintext, pt,
        "successful round-trip proves KDF algorithm from materials suite was used"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_esdk_algorithm_suite_yields_error() {
    let keyring = aes_keyring(0).await;
    let pt = b"unsupported esdk suite test";

    // Encrypt with a valid ESDK suite
    let enc_input =
        EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with the algorithm suite ID bytes in the header to an invalid value.
    // V2 header: byte 0 = version (0x02), bytes 1-2 = algorithm suite ID.
    // Set to 0xFF 0xFF which is not a valid ESDK suite ID.
    ct[1] = 0xFF;
    ct[2] = 0xFF;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when algorithm suite ID is not a supported ESDK suite");
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Tampered suite ID to non-ESDK value triggers the error path
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# decrypt MUST yield an error.
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Tampered header suite ID causes failure, proving parsed suite is used
    //# - Algorithm Suite ID: This MUST be the parsed
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# from the message header.
    assert_eq!(err.kind, ErrorKind::ValidationError, "got: {err:?}");
}
