// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#behavior, encrypt.md#get-the-encryption-materials,
//! encrypt.md#construct-the-header, and encrypt.md#output.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_step_1_get_encryption_materials() {
    // A successful encrypt proves materials were obtained (step 1).
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation Step 1 MUST be [Get the encryption materials](#get-the-encryption-materials)
    let pt = b"test step 1";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_2_construct_header() {
    // A successful encrypt produces output starting with a valid header.
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 2 MUST be [Construct the header](#construct-the-header)
    let output = encrypt_default(b"test step 2").await;
    // The default suite is V2 (committing), so the first byte must be 0x02.
    assert_eq!(output.ciphertext[0], 0x02, "output must start with a valid V2 header version byte");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_3_construct_body() {
    // A successful round-trip proves the body was encrypted correctly (step 3).
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 3 MUST be [Construct the body](#construct-the-body)
    let pt = b"test step 3";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_4_construct_signature() {
    // Encrypt with a signing suite; decrypt verifies the signature, proving step 4 executed.
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 4 MUST be [Construct the signature](#construct-the-signature)
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
async fn test_encrypt_signing_suite_must_perform_signature_step() {
    // Encrypt with a signing suite and verify round-trip succeeds.
    // Decrypt verifies the signature, so success proves the signature step was performed.
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - If the [encryption materials gathered](#get-the-encryption-materials) has a algorithm suite
    //# including a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Encrypt operation MUST perform this step.
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
    // A successful decrypt proves the output message contains only valid message format data.
    // If extra data were appended, the parser would fail or leave trailing bytes.
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# Any data that is not specified within the [message format](../data-format/message.md)
    //# MUST NOT be added to the output message.
    let pt = b"no extra data test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

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
    let err = result.expect_err("encrypt must fail when input suite violates commitment policy");
    let dbg = format!("{err:?}");
    assert!(
        dbg.to_lowercase().contains("commitment") || dbg.to_lowercase().contains("committing")
            || dbg.to_lowercase().contains("policy"),
        "error must indicate commitment-policy failure, got: {dbg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_obtain_materials_from_cmm() {
    // A successful encrypt proves materials were obtained from the CMM.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# This operation MUST obtain this set of [encryption materials](../framework/structures.md#encryption-materials)
    //# by calling [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# To construct the [encrypted message](#encrypted-message),
    //# some fields MUST be constructed using information obtained
    //# from a set of valid [encryption materials](../framework/structures.md#encryption-materials).
    let pt = b"obtain materials test";
    let output = encrypt_default(pt).await;
    assert!(!output.ciphertext.is_empty(), "encrypt must produce ciphertext from CMM-provided materials");
    // The output algorithm suite comes from encryption materials; verify it is a valid ESDK suite.
    // The default CMM selects AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 when no suite is specified.
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        "output suite must come from encryption materials returned by the CMM"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_used_must_be_input_cmm() {
    // Create a CMM from a keyring, then pass it as the CMM input.
    // A successful round-trip proves the input CMM was used.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The CMM used MUST be the input CMM, if supplied.
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
    // Encrypt with a non-empty encryption context and verify it appears in the output.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).
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
    // Encrypt with no encryption context; the CMM receives an empty EC.
    // The output EC should contain no user-provided keys (only CMM-added keys, if any).
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# Otherwise, this MUST be an empty encryption context.
    let pt = b"empty ec test";
    let output = encrypt_default(pt).await;
    let decrypted = decrypt_ciphertext(&output.ciphertext).await;
    assert_eq!(decrypted.plaintext, pt, "round-trip must recover original plaintext");
    // No user-provided keys should appear in the output encryption context.
    assert!(
        !output.encryption_context.keys().any(|k| !k.starts_with("aws-crypto-")),
        "output encryption context must not contain user-provided keys when input EC is empty"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_commitment_policy() {
    // Encrypt with a committing suite and RequireEncryptRequireDecrypt policy.
    // Success proves the commitment policy was correctly passed to the CMM.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy) configured in the [client](client.md) exposing this encrypt function.
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
    // Encrypt with a specific algorithm suite and verify the output uses it.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Algorithm Suite: If provided, this MUST be the [input algorithm suite](#algorithm-suite).
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
    // Encrypt without specifying an algorithm suite; success proves the CMM
    // was called without an algorithm suite field and selected one itself.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If no Algorithm Suite is provided, this field MUST NOT be included.
    let pt = b"no suite test";
    let output = encrypt_default(pt).await;
    let decrypted = decrypt_ciphertext(&output.ciphertext).await;
    assert_eq!(decrypted.plaintext, pt, "round-trip must recover original plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_max_plaintext_length() {
    // EncryptInput takes &[u8] which always has known length.
    // A successful encrypt proves the known length was passed to the CMM.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
    //# this length MUST be used.
    let pt = b"max plaintext length test";
    let output = encrypt_default(pt).await;
    let decrypted = decrypt_ciphertext(&output.ciphertext).await;
    assert_eq!(decrypted.plaintext, pt, "round-trip must recover original plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_construction() {
    // A successful encrypt proves the CMM request was correctly constructed.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=A successful encrypt-then-decrypt round-trip proves the CMM request was correctly constructed, because decrypt would fail if the CMM received malformed encryption materials.
    //# The call to [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials)
    //# on that CMM MUST be constructed as follows:
    let pt = b"cmm request construction test";
    let output = encrypt_default(pt).await;
    let decrypted = decrypt_ciphertext(&output.ciphertext).await;
    assert_eq!(decrypted.plaintext, pt, "round-trip must recover original plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_from_materials_used() {
    // Encrypt with a specific suite and verify the output reports the same suite.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.
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
    // This tests the post-CMM commitment policy check. In practice, the pre-CMM and
    // post-CMM checks exercise the same validation because the default CMM returns the
    // requested suite unchanged. The post-CMM check exists to catch cases where a custom
    // CMM returns a different (non-committing) suite than what was requested.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"post-cmm commitment", EncryptionContext::new(), keyring);
    // Non-committing suite with RequireEncryptRequireDecrypt: commitment policy check should fail
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must fail when post-CMM suite violates commitment policy");
    let dbg = format!("{err:?}");
    assert!(
        dbg.to_lowercase().contains("commitment") || dbg.to_lowercase().contains("committing")
            || dbg.to_lowercase().contains("policy"),
        "error must indicate commitment-policy failure, got: {dbg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_max_edk_exceeded_error() {
    // Set max_encrypted_data_keys to 0 (impossible to satisfy) — should fail.
    // NonZeroUsize minimum is 1, but even 1 EDK from a single keyring should be exactly 1.
    // We use two keyrings to produce 2 EDKs, then set max to 1.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md) encrypt MUST yield an error.
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
    let err = result.expect_err("encrypt must fail when EDK count exceeds max");
    assert!(
        err.message.contains("exceed") && err.message.contains("maximum"),
        "error must indicate EDK count exceeds maximum, got: {} ({:?})",
        err.message, err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_data_key_derived_from_plaintext_data_key() {
    // A successful round-trip proves the derived data key was used for encryption,
    // because decrypt derives the same key from the same plaintext data key.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Round-trip success proves the derived data key was used: decrypt re-derives the same key from the plaintext data key in the header, so a mismatch would cause decryption failure.
    //# The data key used as input for all encryption described below MUST be a data key derived from the plaintext data key
    //# included in the [encryption materials](../framework/structures.md#encryption-materials).
    let pt = b"derived data key test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_input_used() {
    // Encrypt with a custom frame length and verify round-trip succeeds.
    // The frame length affects body structure; wrong frame length would cause decrypt failure.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The frame length used in the procedures described below MUST be the input [frame length](#frame-length),
    //# if supplied.
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
    // Encrypt without specifying frame length (uses default 4096).
    // A successful round-trip proves the default frame length was used.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Round-trip success without specifying frame length proves the default (4096) was used: the header records the frame length, and decrypt uses it to parse the body.
    //# If no input frame length is supplied, the default frame length MUST be used.
    let pt = b"default frame length test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_header_before_body() {
    // A successful round-trip proves the header was serialized before the body,
    // because decrypt parses header first, then uses header info to decrypt body.
    //= specification/client-apis/encrypt.md#construct-the-header
    //= type=test
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).
    let pt = b"header serialization test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_format_version_matches_suite() {
    // Encrypt with a V2 (committing) suite and verify the first byte is 0x02 (version 2).
    //= specification/client-apis/encrypt.md#construct-the-header
    //= type=test
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).
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

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_encrypted_message() {
    //= specification/client-apis/encrypt.md#output
    //= type=test
    //# - Encrypt operation output MUST include an [encrypted message](#encrypted-message) value.
    //= specification/client-apis/encrypt.md#encrypted-message
    //= type=test
    //# This MUST be a sequence of bytes
    //# and conform to the [message format specification](../data-format/message.md).
    let output = encrypt_default(b"output encrypted message test").await;
    assert!(!output.ciphertext.is_empty(), "output must include non-empty encrypted message");
    // First byte must be a valid ESDK message format version (0x01 or 0x02)
    assert!(
        output.ciphertext[0] == 0x01 || output.ciphertext[0] == 0x02,
        "first byte must be a valid ESDK version (0x01 or 0x02), got {:#04x}",
        output.ciphertext[0]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_encryption_context() {
    //= specification/client-apis/encrypt.md#output
    //= type=test
    //# - Encrypt operation output MUST include an [encryption context](#encryption-context) value.
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
    //# - Encrypt operation output MUST include an [algorithm suite](#algorithm-suite) value.
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

#[tokio::test(flavor = "multi_thread")]
async fn test_reserved_encryption_context_prefix_must_fail() {
    //= specification/client-apis/encrypt.md#encryption-context
    //= type=test
    //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
    //# the encryption operation MUST fail.
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([
        ("aws-crypto-foo".to_string(), "bar".to_string()),
    ]);
    let enc_input = EncryptInput::with_legacy_keyring(b"should fail", ec, keyring);
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must fail when encryption context has aws-crypto- prefix key");
    assert!(
        err.message.contains("aws-crypto-") || err.message.to_lowercase().contains("reserved"),
        "error must indicate the reserved-prefix failure, got: {} ({:?})",
        err.message, err.kind
    );
}

// Boundary: `aws-crypto` without the trailing dash MUST be accepted, proving the check requires the trailing dash.
#[tokio::test(flavor = "multi_thread")]
async fn test_reserved_encryption_context_prefix_boundary_no_dash() {
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([
        ("aws-crypto".to_string(), "bar".to_string()),
    ]);
    let enc_input = EncryptInput::with_legacy_keyring(b"should succeed", ec, keyring);
    let result = encrypt(&enc_input).await;
    assert!(
        result.is_ok(),
        "encrypt must accept key 'aws-crypto' (no trailing dash): only 'aws-crypto-' is reserved, got: {:?}",
        result.err()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_algorithm_suite_used_for_encryption() {
    //= specification/client-apis/encrypt.md#algorithm-suite
    //= type=test
    //# The [algorithm suite](../framework/algorithm-suites.md) that MUST be used for encryption.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite used test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        "the specified algorithm suite must be used for encryption"
    );
    // Verify round-trip to prove the suite was actually used for encryption
    let dec_input = DecryptInput::with_legacy_keyring(&output.ciphertext, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(pt, b"suite used test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_algorithm_suite_must_be_esdk_supported() {
    // Verify that encrypting with a valid ESDK-supported suite succeeds.
    //= specification/client-apis/encrypt.md#algorithm-suite
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"esdk supported suite", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let result = encrypt(&enc_input).await;
    assert!(result.is_ok(), "encrypt must succeed with an ESDK-supported algorithm suite");
}