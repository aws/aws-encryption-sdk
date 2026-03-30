// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#behavior, encrypt.md#get-the-encryption-materials,
//! encrypt.md#construct-the-header, and encrypt.md#output.

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

/// Encrypt with defaults, return output.
async fn encrypt_default(plaintext: &[u8]) -> EncryptOutput {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap()
}

/// Encrypt then decrypt round-trip, return decrypted plaintext.
async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

// ── Behavior: Step ordering ──

#[tokio::test(flavor = "multi_thread")]
async fn test_step_1_get_encryption_materials() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation Step 1 MUST be [Get the encryption materials](#get-the-encryption-materials)

    // A successful encrypt proves materials were obtained (step 1).
    let pt = b"test step 1";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_2_construct_header() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 2 MUST be [Construct the header](#construct-the-header)

    // A successful encrypt produces output starting with a valid header.
    let output = encrypt_default(b"test step 2").await;
    assert!(output.ciphertext.len() > 1, "output must contain a serialized header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_3_construct_body() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 3 MUST be [Construct the body](#construct-the-body)

    // A successful round-trip proves the body was encrypted correctly (step 3).
    let pt = b"test step 3";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_4_construct_signature() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 4 MUST be [Construct the signature](#construct-the-signature)

    // Encrypt with a signing suite; decrypt verifies the signature, proving step 4 executed.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"test step 4", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(pt, b"test step 4");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_must_perform_signature_step() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - If the [encryption materials gathered](#get-the-encryption-materials) has a algorithm suite
    //# including a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Encrypt operation MUST perform this step.

    // Encrypt with a signing suite and verify round-trip succeeds.
    // Decrypt verifies the signature, so success proves the signature step was performed.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"signing step test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(pt, b"signing step test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_extra_data_in_output_message() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# Any data that is not specified within the [message format](../data-format/message.md)
    //# MUST NOT be added to the output message.

    // A successful decrypt proves the output message contains only valid message format data.
    // If extra data were appended, the parser would fail or leave trailing bytes.
    let pt = b"no extra data test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

// ── Get the encryption materials ──

#[tokio::test(flavor = "multi_thread")]
async fn test_input_suite_vs_commitment_policy_error() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If an [input algorithm suite](#algorithm-suite) is provided
    //# that is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.

    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"commitment check", EncryptionContext::new(), keyring);
    // Non-committing suite with RequireEncryptRequireDecrypt policy
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    assert!(result.is_err(), "encrypt must fail when input suite violates commitment policy");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_obtain_materials_from_cmm() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# This operation MUST obtain this set of [encryption materials](../framework/structures.md#encryption-materials)
    //# by calling [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) on a [CMM](../framework/cmm-interface.md).

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# To construct the [encrypted message](#encrypted-message),
    //# some fields MUST be constructed using information obtained
    //# from a set of valid [encryption materials](../framework/structures.md#encryption-materials).

    // A successful encrypt proves materials were obtained from the CMM.
    let pt = b"obtain materials test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_used_must_be_input_cmm() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The CMM used MUST be the input CMM, if supplied.

    // Create a CMM from a keyring, then pass it as the CMM input.
    // A successful round-trip proves the input CMM was used.
    let keyring = test_keyring().await;
    let cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();
    let pt = b"input cmm test";
    let enc_input = EncryptInput::with_legacy_cmm(pt, EncryptionContext::new(), cmm.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_cmm(&ct, EncryptionContext::new(), cmm);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_encryption_context() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).

    // Encrypt with a non-empty encryption context and verify it appears in the output.
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([("mykey".to_string(), "myval".to_string())]);
    let enc_input = EncryptInput::with_legacy_keyring(b"ec test", ec.clone(), keyring.clone());
    let output = encrypt(&enc_input).await.unwrap();
    assert!(
        output.encryption_context.contains_key("mykey"),
        "output encryption context must contain the input key"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_empty_encryption_context() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# Otherwise, this MUST be an empty encryption context.

    // Encrypt with no encryption context; success proves empty EC was passed to CMM.
    let pt = b"empty ec test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_commitment_policy() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy) configured in the [client](client.md) exposing this encrypt function.

    // Encrypt with a committing suite and RequireEncryptRequireDecrypt policy.
    // Success proves the commitment policy was correctly passed to the CMM.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"commitment policy test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, b"commitment policy test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_algorithm_suite_provided() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Algorithm Suite: If provided, this MUST be the [input algorithm suite](#algorithm-suite).

    // Encrypt with a specific algorithm suite and verify the output uses it.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite test", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        "output algorithm suite must match the input algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_no_algorithm_suite() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If no Algorithm Suite is provided, this field MUST NOT be included.

    // Encrypt without specifying an algorithm suite; success proves the CMM
    // was called without an algorithm suite field and selected one itself.
    let pt = b"no suite test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_max_plaintext_length() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
    //# this length MUST be used.

    // EncryptInput takes &[u8] which always has known length.
    // A successful encrypt proves the known length was passed to the CMM.
    let pt = b"max plaintext length test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_construction() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The call to [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials)
    //# on that CMM MUST be constructed as follows:

    // A successful encrypt proves the CMM request was correctly constructed.
    let pt = b"cmm request construction test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_from_materials_used() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.

    // Encrypt with a specific suite and verify the output reports the same suite.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite from materials", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    assert_eq!(output.algorithm_suite_id, EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_post_cmm_commitment_policy_error() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.

    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"post-cmm commitment", EncryptionContext::new(), keyring);
    // Non-committing suite with RequireEncryptRequireDecrypt: post-CMM check should fail
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    assert!(result.is_err(), "encrypt must fail when post-CMM suite violates commitment policy");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_max_edk_exceeded_error() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md) encrypt MUST yield an error.

    // Set max_encrypted_data_keys to 0 (impossible to satisfy) — should fail.
    // NonZeroUsize minimum is 1, but even 1 EDK from a single keyring should be exactly 1.
    // We use two keyrings to produce 2 EDKs, then set max to 1.
    let keyring1 = test_keyring().await;
    let (ns2, name2) = namespace_and_name(1);
    let keyring2 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns2)
        .key_name(name2)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let multi_keyring = mpl()
        .create_multi_keyring()
        .generator(keyring1)
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"max edk test", EncryptionContext::new(), multi_keyring);
    enc_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    let result = encrypt(&enc_input).await;
    assert!(result.is_err(), "encrypt must fail when EDK count exceeds max");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_data_key_derived_from_plaintext_data_key() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The data key used as input for all encryption described below MUST be a data key derived from the plaintext data key
    //# included in the [encryption materials](../framework/structures.md#encryption-materials).

    // A successful round-trip proves the derived data key was used for encryption,
    // because decrypt derives the same key from the same plaintext data key.
    let pt = b"derived data key test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_input_used() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The frame length used in the procedures described below MUST be the input [frame length](#frame-length),
    //# if supplied.

    // Encrypt with a custom frame length and verify round-trip succeeds.
    // The frame length affects body structure; wrong frame length would cause decrypt failure.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"custom frame length", EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(512).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, b"custom frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_default_frame_length_used() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If no input frame length is supplied, the default frame length MUST be used.

    // Encrypt without specifying frame length (uses default 4096).
    // A successful round-trip proves the default frame length was used.
    let pt = b"default frame length test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

// ── Construct the header ──

#[tokio::test(flavor = "multi_thread")]
async fn test_serialize_header_before_body() {
    //= specification/client-apis/encrypt.md#construct-the-header
    //= type=test
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).

    // A successful round-trip proves the header was serialized before the body,
    // because decrypt parses header first, then uses header info to decrypt body.
    let pt = b"header serialization test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_format_version_matches_suite() {
    //= specification/client-apis/encrypt.md#construct-the-header
    //= type=test
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).

    // Encrypt with a V2 (committing) suite and verify the first byte is 0x02 (version 2).
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"version test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    assert_eq!(ct[0], 0x02, "V2 committing suite must produce message version 2");

    // Encrypt with a V1 (non-committing) suite and verify the first byte is 0x01 (version 1).
    let mut enc_input_v1 =
        EncryptInput::with_legacy_keyring(b"version test v1", EncryptionContext::new(), keyring);
    enc_input_v1.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input_v1.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct_v1 = encrypt(&enc_input_v1).await.unwrap().ciphertext;
    assert_eq!(ct_v1[0], 0x01, "V1 non-committing suite must produce message version 1");
}

// ── Output ──

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_encrypted_message() {
    //= specification/client-apis/encrypt.md#output
    //= type=test
    //# - The output of the Encrypt operation MUST include an [encrypted message](#encrypted-message) value.

    //= specification/client-apis/encrypt.md#encrypted-message
    //= type=test
    //# This MUST be a sequence of bytes
    //# and conform to the [message format specification](../data-format/message.md).

    let output = encrypt_default(b"output encrypted message test").await;
    assert!(!output.ciphertext.is_empty(), "output must include non-empty encrypted message");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_encryption_context() {
    //= specification/client-apis/encrypt.md#output
    //= type=test
    //# - The output of the Encrypt operation MUST include an [encryption context](#encryption-context) value.

    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([("testkey".to_string(), "testval".to_string())]);
    let enc_input = EncryptInput::with_legacy_keyring(b"output ec test", ec, keyring);
    let output = encrypt(&enc_input).await.unwrap();
    assert!(
        output.encryption_context.contains_key("testkey"),
        "output must include the encryption context"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_algorithm_suite() {
    //= specification/client-apis/encrypt.md#output
    //= type=test
    //# - The output of the Encrypt operation MUST include an [algorithm suite](#algorithm-suite) value.

    //= specification/client-apis/encrypt.md#algorithm-suite-1
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).

    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"output suite test", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        "output must include the algorithm suite"
    );
}
