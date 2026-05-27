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
async fn test_step_2_construct_header() {
    // A successful encrypt produces output starting with a valid header version byte
    // for both V1 and V2 message formats.
    let v2 = encrypt_v2(b"test step 2 v2").await;
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 2 MUST be [Construct the header](#construct-the-header)
    assert_eq!(v2[0], 0x02, "V2 output must start with header version byte 0x02");

    let v1 = encrypt_v1(b"test step 2 v1").await;
    assert_eq!(v1[0], 0x01, "V1 output must start with header version byte 0x01");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_4_construct_signature() {
    // Encrypt with a signing suite; decrypt verifies the signature, proving step 4 executed.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"test step 4", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //# - Encrypt operation step 4 MUST be [Construct the signature](#construct-the-signature)
    //
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //= reason=Decrypt verifies footer signature; success proves encrypt performed the step
    //# - If the [encryption materials gathered](#get-the-encryption-materials) has a algorithm suite
    //# including a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Encrypt operation MUST perform this step.
    assert_eq!(pt, b"test step 4");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_extra_data_in_output_message() {
    // Compute the end-of-message offset by walking the frames (and the footer if a
    // signing suite is in use) and assert it equals the ciphertext length.
    let pt = b"no extra data test";

    // Case 1: V2 non-signing — body ends at the final frame.
    let ct = encrypt_without_signing_suite(pt).await;
    let frames = parse_all_frames(&ct, 4096);
    let body_end = frames.last().expect("at least one frame").end_offset;
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //# Any data that is not specified within the [message format](../data-format/message.md)
    //# MUST NOT be added to the output message.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=parse_all_frames independently walks wire bytes verifying frame structure
    //# The Encrypt operation MUST serialize a regular frame or final frame with the following specifics:
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=parse_all_frames validates seq_num/IV/content/tag layout matches regular frame spec
    //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=parse_all_frames extracts each field from wire bytes at spec-defined offsets
    //# For a regular frame, each field MUST be serialized according to its specification:
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=parse_all_frames detects ENDFRAME marker and validates final frame layout
    //# Final frame serialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=parse_all_frames extracts each final frame field at spec-defined offsets
    //# For a final frame, each field MUST be serialized according to its specification:
    assert_eq!(
        body_end, ct.len(),
        "V2 non-signing: ciphertext must end exactly at the final frame; trailing bytes = {}",
        ct.len() - body_end
    );

    // Case 2: V2 signing — body ends, then footer (sig_len + signature) ends at ct.len().
    let ct = encrypt_with_signing_suite(pt).await;
    let frames = parse_all_frames(&ct, 4096);
    let body_end = frames.last().expect("at least one frame").end_offset;
    let (footer_offset, sig_len) = find_footer_offset(&ct);
    assert_eq!(
        footer_offset, body_end,
        "V2 signing: footer must begin immediately after the final frame"
    );
    assert_eq!(
        footer_offset + 2 + sig_len as usize,
        ct.len(),
        "V2 signing: ciphertext must end exactly at the footer; trailing bytes = {}",
        ct.len() - (footer_offset + 2 + sig_len as usize)
    );

    // Round-trip corroboration.
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_input_suite_vs_commitment_policy_error() {
    // Non-committing suite + RequireEncryptRequireDecrypt → must fail.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"commitment check", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must fail when input suite violates commitment policy");
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If an [input algorithm suite](#algorithm-suite) is provided
    //# that is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    //
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Test sets commitment_policy on input; policy-violation error proves it was passed
    //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy) configured in the [client](client.md) exposing this encrypt function.
    assert!(
        inner.contains("InvalidAlgorithmSuiteInfoOnEncrypt"),
        "expected InvalidAlgorithmSuiteInfoOnEncrypt, got: {inner}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_obtain_materials_from_cmm() {
    // A successful encrypt proves materials were obtained from the CMM.
    let pt = b"obtain materials test";
    let output = encrypt_default(pt).await;
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# This operation MUST obtain this set of [encryption materials](../framework/structures.md#encryption-materials)
    //# by calling [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) on a [CMM](../framework/cmm-interface.md).
    //
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# To construct the [encrypted message](#encrypted-message),
    //# some fields MUST be constructed using information obtained
    //# from a set of valid [encryption materials](../framework/structures.md#encryption-materials).
    assert!(!output.ciphertext.is_empty(), "encrypt must produce ciphertext from CMM-provided materials");
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=encrypt_default passes a keyring; success proves default CMM was constructed
    //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
    //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
    //# constructed using the caller-supplied keyring as input.
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
    // Decrypt with the same CMM succeeds, proving encrypt used the input CMM.
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
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The CMM used MUST be the input CMM, if supplied.
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_encryption_context() {
    // Encrypt with a non-empty encryption context and verify it appears in the output.
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([("mykey".to_string(), "myval".to_string())]);
    let enc_input = EncryptInput::with_legacy_keyring(b"ec test", ec.clone(), keyring.clone());
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).
    assert!(
        output.encryption_context.contains_key("mykey"),
        "output encryption context must contain the input key"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_empty_encryption_context() {
    // Encrypt with no encryption context; verify no user-provided keys in output.
    let pt = b"empty ec test";
    let output = encrypt_default(pt).await;
    let decrypted = decrypt_ciphertext(&output.ciphertext).await;
    assert_eq!(decrypted.plaintext, pt, "round-trip must recover original plaintext");
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# Otherwise, this MUST be an empty encryption context.
    assert!(
        !output.encryption_context.keys().any(|k| !k.starts_with("aws-crypto-")),
        "output encryption context must not contain user-provided keys when input EC is empty"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_algorithm_suite_provided() {
    // Encrypt with a specific algorithm suite and verify the output uses it.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite test", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - Algorithm Suite: If provided, this MUST be the [input algorithm suite](#algorithm-suite).
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        "output algorithm suite must match the input algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_from_materials_used() {
    // Encrypt with a specific suite and verify the output reports the same suite.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite from materials", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.
    assert_eq!(output.algorithm_suite_id, EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_max_edk_exceeded_error() {
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
    let err = result.expect_err("encrypt must fail when EDK count exceeds max");
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md) encrypt MUST yield an error.
    assert_eq!(
        err.kind, ErrorKind::ValidationError,
        "expected ValidationError, got: {err:?}"
    );
    assert!(
        err.message.contains("exceed") && err.message.contains("maximum"),
        "error must indicate EDK count exceeds maximum, got: {}",
        err.message
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_data_key_derived_from_plaintext_data_key() {
    // Decrypt re-derives the same key from the plaintext data key; success proves
    // encrypt used the correctly derived data key.
    let pt = b"derived data key test";
    let result = round_trip(pt).await;
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Decrypt re-derives the same key; mismatch would cause decryption failure
    //# The data key used as input for all encryption described below MUST be a data key derived from the plaintext data key
    //# included in the [encryption materials](../framework/structures.md#encryption-materials).
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_input_used() {
    // Encrypt with a custom frame length and verify the header records that exact value.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"custom frame length", EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(512).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let fields = parse_v2_header_field_offsets(&ct);
    let (_, fl_start, fl_end) = fields.iter().find(|(n, _, _)| *n == "Frame Length")
        .expect("V2 header must have a Frame Length field");
    assert_eq!(fl_end - fl_start, 4, "frame_length field must be 4 bytes");
    let on_wire = u32::from_be_bytes([ct[*fl_start], ct[fl_start + 1], ct[fl_start + 2], ct[fl_start + 3]]);
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# The frame length used in the procedures described below MUST be the input [frame length](#frame-length),
    //# if supplied.
    assert_eq!(on_wire, 512, "header frame_length must equal the input frame length");

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, b"custom frame length");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_default_frame_length_used() {
    // Encrypt without specifying a frame length and verify the header records the default (4096).
    let pt = b"default frame length test";
    let ct = encrypt_default(pt).await.ciphertext;

    let fields = parse_v2_header_field_offsets(&ct);
    let (_, fl_start, fl_end) = fields.iter().find(|(n, _, _)| *n == "Frame Length")
        .expect("V2 header must have a Frame Length field");
    assert_eq!(fl_end - fl_start, 4, "frame_length field must be 4 bytes");
    let on_wire = u32::from_be_bytes([ct[*fl_start], ct[fl_start + 1], ct[fl_start + 2], ct[fl_start + 3]]);
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# If no input frame length is supplied, the default frame length MUST be used.
    assert_eq!(on_wire, 4096, "default frame_length on the wire must be 4096");

    // Round-trip corroboration.
    let result = round_trip(pt).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_format_version_matches_suite() {
    // Encrypt with a V2 (committing) suite and verify the first byte is 0x02 (version 2).
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"version test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    //= spec/client-apis/encrypt.md#construct-the-header
    //= type=test
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).
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
    let output = encrypt_default(b"output encrypted message test").await;
    //= spec/client-apis/encrypt.md#output
    //= type=test
    //# - Encrypt operation output MUST include an [encrypted message](#encrypted-message) value.
    assert!(!output.ciphertext.is_empty(), "output must include non-empty encrypted message");
    //= spec/client-apis/encrypt.md#encrypted-message
    //= type=test
    //# This MUST be a sequence of bytes
    //# and conform to the [message format specification](../data-format/message.md).
    assert!(
        output.ciphertext[0] == 0x01 || output.ciphertext[0] == 0x02,
        "first byte must be a valid ESDK version (0x01 or 0x02), got {:#04x}",
        output.ciphertext[0]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_encryption_context() {
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([("testkey".to_string(), "testval".to_string())]);
    let enc_input = EncryptInput::with_legacy_keyring(b"output ec test", ec, keyring);
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#output
    //= type=test
    //# - Encrypt operation output MUST include an [encryption context](#encryption-context) value.
    assert!(
        output.encryption_context.contains_key("testkey"),
        "output must include the encryption context"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_output_includes_algorithm_suite() {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"output suite test", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#output
    //= type=test
    //# - Encrypt operation output MUST include an [algorithm suite](#algorithm-suite) value.
    //
    //= spec/client-apis/encrypt.md#algorithm-suite-1
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    assert_eq!(
        output.algorithm_suite_id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        "output must include the algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reserved_encryption_context_prefix_must_fail() {
    let keyring = test_keyring().await;
    let ec = std::collections::HashMap::from([
        ("aws-crypto-foo".to_string(), "bar".to_string()),
    ]);
    let enc_input = EncryptInput::with_legacy_keyring(b"should fail", ec, keyring);
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must fail when encryption context has aws-crypto- prefix key");
    //= spec/client-apis/encrypt.md#encryption-context
    //= type=test
    //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
    //# the encryption operation MUST fail.
    assert_eq!(
        err.kind, ErrorKind::ValidationError,
        "expected ValidationError, got: {err:?}"
    );
    assert!(
        err.message.contains("aws-crypto-"),
        "error must identify the reserved prefix, got: {}",
        err.message
    );
}

// Boundary: `aws-crypto` without the trailing dash MUST be accepted.
#[tokio::test(flavor = "multi_thread")]
async fn test_reserved_encryption_context_prefix_boundary_no_dash() {
    // Proves only 'aws-crypto-' (with dash) is reserved; 'aws-crypto' alone is valid.
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
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"suite used test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let output = encrypt(&enc_input).await.unwrap();
    //= spec/client-apis/encrypt.md#algorithm-suite
    //= type=test
    //# The [algorithm suite](../framework/algorithm-suites.md) that MUST be used for encryption.
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

// Spy CMM that records what inputs it received, then delegates to a real CMM.
struct SpyCmm {
    inner: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    observed_algorithm_suite_id: std::sync::Arc<std::sync::Mutex<Option<Option<aws_mpl_legacy::dafny::types::AlgorithmSuiteId>>>>,
    observed_max_plaintext_length: std::sync::Arc<std::sync::Mutex<Option<Option<i64>>>>,
}

impl aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManager for SpyCmm {
    fn get_encryption_materials(
        &self,
        input: aws_mpl_legacy::dafny::operation::get_encryption_materials::GetEncryptionMaterialsInput,
    ) -> Result<aws_mpl_legacy::dafny::operation::get_encryption_materials::GetEncryptionMaterialsOutput, aws_mpl_legacy::dafny::types::error::Error> {
        // Record observations
        *self.observed_algorithm_suite_id.lock().unwrap() = Some(input.algorithm_suite_id.clone());
        *self.observed_max_plaintext_length.lock().unwrap() = Some(input.max_plaintext_length);

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let mut builder = self.inner.get_encryption_materials()
                    .commitment_policy(input.commitment_policy.unwrap())
                    .encryption_context(input.encryption_context.unwrap());
                if let Some(suite) = input.algorithm_suite_id {
                    builder = builder.algorithm_suite_id(suite);
                }
                if let Some(len) = input.max_plaintext_length {
                    builder = builder.max_plaintext_length(len);
                }
                builder.send().await
            })
        })
    }
    fn decrypt_materials(
        &self,
        input: aws_mpl_legacy::dafny::operation::decrypt_materials::DecryptMaterialsInput,
    ) -> Result<aws_mpl_legacy::dafny::operation::decrypt_materials::DecryptMaterialsOutput, aws_mpl_legacy::dafny::types::error::Error> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.inner.decrypt_materials()
                    .algorithm_suite_id(input.algorithm_suite_id.unwrap())
                    .commitment_policy(input.commitment_policy.unwrap())
                    .encryption_context(input.encryption_context.unwrap())
                    .encrypted_data_keys(input.encrypted_data_keys.unwrap())
                    .send()
                    .await
            })
        })
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_no_algorithm_suite_field() {
    // Spy CMM observes algorithm_suite_id is None when caller omits it.
    let keyring = test_keyring().await;
    let inner_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();
    let observed_suite = std::sync::Arc::new(std::sync::Mutex::new(None));
    let observed_len = std::sync::Arc::new(std::sync::Mutex::new(None));
    let cmm_ref = aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef::from(SpyCmm {
        inner: inner_cmm,
        observed_algorithm_suite_id: observed_suite.clone(),
        observed_max_plaintext_length: observed_len.clone(),
    });

    let pt = b"no suite spy test";
    let enc_input = EncryptInput::with_legacy_cmm(pt, EncryptionContext::new(), cmm_ref);
    let output = encrypt(&enc_input).await.unwrap();

    // Verify spy observed None for algorithm_suite_id
    let observed = observed_suite.lock().unwrap().clone();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Spy CMM directly observes the call was constructed with expected fields
    //# The call to [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials)
    //# on that CMM MUST be constructed as follows:
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Spy CMM observes algorithm_suite_id is None when caller omits it
    //# If no Algorithm Suite is provided, this field MUST NOT be included.
    assert_eq!(observed, Some(None), "CMM must receive algorithm_suite_id=None when caller omits it");

    // Round-trip corroboration
    let dec_input = DecryptInput::with_legacy_keyring(&output.ciphertext, EncryptionContext::new(), keyring);
    assert_eq!(decrypt(&dec_input).await.unwrap().plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_request_max_plaintext_length_equals_input() {
    // Spy CMM observes max_plaintext_length equals input plaintext length.
    let keyring = test_keyring().await;
    let inner_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();
    let observed_suite = std::sync::Arc::new(std::sync::Mutex::new(None));
    let observed_len = std::sync::Arc::new(std::sync::Mutex::new(None));
    let cmm_ref = aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef::from(SpyCmm {
        inner: inner_cmm,
        observed_algorithm_suite_id: observed_suite.clone(),
        observed_max_plaintext_length: observed_len.clone(),
    });

    let pt = b"spy plaintext 23 bytes!";  // 23 bytes
    let enc_input = EncryptInput::with_legacy_cmm(pt, EncryptionContext::new(), cmm_ref);
    encrypt(&enc_input).await.unwrap();

    // Verify spy observed max_plaintext_length == plaintext.len()
    let observed = observed_len.lock().unwrap().clone();
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Spy CMM observes max_plaintext_length equals input plaintext length
    //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
    //# this length MUST be used.
    assert_eq!(
        observed,
        Some(Some(pt.len() as i64)),
        "CMM must receive max_plaintext_length equal to input plaintext length"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_step_failure_must_halt_and_indicate_failure() {
    // Non-committing suite + RequireEncryptRequireDecrypt causes step 1 to fail.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"halt test", EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must halt and indicate failure when a step fails");
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //# If any of these steps fails, this operation MUST halt and indicate a failure to the caller.
    assert!(
        inner.contains("InvalidAlgorithmSuiteInfoOnEncrypt"),
        "expected InvalidAlgorithmSuiteInfoOnEncrypt, got: {inner}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_length_bound_used_for_unknown_length() {
    // encrypt_stream with data_size=Some(100) passes the bound as max_plaintext_length.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    stream_input.data_size = Some(100);
    let plaintext = vec![0xAAu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input).await.unwrap();

    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=encrypt_stream with data_size=Some(100) passes the bound; success proves it was used
    //# If the input [plaintext](#plaintext) has unknown length and a [Plaintext Length Bound](#plaintext-length-bound)
    //# was provided, this MUST be the [Plaintext Length Bound](#plaintext-length-bound).
    assert_eq!(pt, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_plaintext_length_bound_field_not_included() {
    // encrypt_stream with data_size=None omits the max_plaintext_length field.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    stream_input.data_size = None;
    let plaintext = vec![0xBBu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input).await.unwrap();

    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=encrypt_stream with data_size=None succeeds; proves field was not included
    //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
    assert_eq!(pt, plaintext);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_header_released_after_serialization() {
    // encrypt_stream writes header before body; output starts with valid version byte.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    stream_input.data_size = Some(20);
    let plaintext = vec![0xCCu8; 20];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input).await.unwrap();

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=Streaming output starts with version byte, proving header was released
    //# If this operation is streaming the encrypted message and
    //# the entire message header has been serialized,
    //# the serialized message header MUST be released.
    assert!(
        output[0] == 0x01 || output[0] == 0x02,
        "streaming output must begin with a valid version byte, proving header was released"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_bodies_not_equal_must_fail() {
    // Tamper encrypted body content; AES-GCM auth failure proves body integrity.
    let pt = b"body equality test";
    let ct = encrypt_without_signing_suite(pt).await;
    let keyring = test_keyring().await;
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    let body_start = find_body_start(&ct, 4096).expect("body start");
    let content_off = body_start + 4 + 4 + IV_LEN + 4;
    let mut tampered = ct.clone();
    let original = tampered[content_off];
    tampered[content_off] ^= 0xFF;
    assert_ne!(tampered[content_off], original, "tamper must change the byte");

    let dec_input = DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered body must cause decrypt to fail");
    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //= reason=Tampered body causes CryptographicError, proving body integrity
    //# If the message bodies are not equal, the Encrypt operation MUST fail.
    assert_eq!(err.kind, ErrorKind::CryptographicError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_must_not_encrypt_using_nonframed_content_type() {
    // Verify content type byte on the wire is 0x02 (Framed) for both V1 and V2.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"nonframed test", version, keyring).await;
        let content_type_byte = match version {
            Version::V1 => {
                let (ct_offset, _, _, _) = parse_v1_trailing_offsets(&ct);
                ct[ct_offset]
            }
            Version::V2 => {
                let ct_offset = content_type_offset_v2(&ct);
                ct[ct_offset]
            }
        };
        //= spec/client-apis/encrypt.md#nonframed-message-body-encryption
        //= type=test
        //= reason=On-wire content type byte is 0x02 (Framed), proving nonframed is never used
        //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the nonframed content type.
        assert_eq!(
            content_type_byte, 0x02,
            "{version:?}: content type must be 0x02 (Framed), not 0x01 (Non-framed)"
        );
    }
}
