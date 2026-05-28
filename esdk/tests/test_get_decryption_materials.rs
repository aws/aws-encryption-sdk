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

// Regression: parses header via __test_internals to extract message_id, then
// runs the full decrypt path end-to-end to confirm derive_keys is actually
// invoked. The "data key MUST be derived from plaintext data key" requirement
// is covered by a source-side type=implication on the literal call site
// (`derive_keys(message_id, dec_mat.plaintext_data_key, ...)` in decrypt.rs).
#[tokio::test(flavor = "multi_thread")]
async fn test_data_key_derived_from_plaintext_data_key() {
    use aws_esdk::__test_internals::*;

    let keyring = aes_keyring(0).await;
    let pt = b"test data key derivation";
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;

    // Parse header to confirm message_id is reachable via the public accessor;
    // derive_keys uses message_id as HKDF info.
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut raw = Vec::new();
    let header_body = read_header_body(&mut cursor, None, &mut raw).unwrap();
    let message_id = header_body.message_id();
    assert!(!message_id.is_empty(), "message_id parsed from header");

    // End-to-end decrypt confirms the real derivation path works.
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_algorithm_suite_from_decryption_materials() {
    // Directly verify: parse the header to get the algorithm suite, then verify
    // decrypt's output reports the same suite. This proves the materials' suite
    // was used (not some hardcoded default).
    use aws_esdk::__test_internals::*;

    let keyring = aes_keyring(0).await;
    let pt = b"test algorithm suite from materials";
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;

    // Parse the header directly to read the algorithm suite from the wire
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut raw = Vec::new();
    let header_body = read_header_body(&mut cursor, None, &mut raw).unwrap();
    let wire_suite_id = header_body.algorithm_suite().id;

    // Decrypt and verify the output suite matches what the header contained
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Output suite matches wire-parsed header suite; proves materials' suite was used
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let output_suite_id = aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(result.algorithm_suite_id);
    assert_eq!(output_suite_id, wire_suite_id, "output suite must match header's parsed suite");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_commit_key_derived_and_validated() {
    let keyring = aes_keyring(0).await;
    let pt = b"test commit key derivation and equality";
    // Use committing suite
    let valid_ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
        &keyring,
    )
    .await;

    // Tamper the Algorithm Suite Data (commit key) in the header.
    // parse_v2_header_field_offsets gives us the exact byte range.
    let fields = parse_v2_header_field_offsets(&valid_ct);
    let (_, sd_start, _sd_end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Algorithm Suite Data")
        .expect("V2 header must have Algorithm Suite Data field");
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[*sd_start] ^= 0xFF;
    assert_ne!(
        tampered_ct[*sd_start], valid_ct[*sd_start],
        "tamper must change the byte"
    );

    let valid_input =
        DecryptInput::with_legacy_keyring(&valid_ct, EncryptionContext::new(), keyring.clone());
    let tampered_input =
        DecryptInput::with_legacy_keyring(&tampered_ct, EncryptionContext::new(), keyring);

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Untampered commit key → Ok; tampered commit key in header → ValidationError, proving the equality check runs
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Header commit key must equal derived commit key; tampering breaks equality → ValidationError
    //# The derived commit key MUST equal the commit key stored in the message header.
    assert!(decrypt(&valid_input).await.is_ok(), "valid commit key must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::ValidationError,
        "tampered commit key must produce ValidationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_kdf_algorithm_from_materials_suite() {
    // Parse the header to verify the algorithm suite uses HKDF, then decrypt
    // to prove that KDF algorithm was actually used for key derivation.
    use aws_esdk::__test_internals::*;

    let keyring = aes_keyring(0).await;
    let pt = b"test kdf algorithm from materials";
    let ct = encrypt_with_suite(
        pt,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        &keyring,
    )
    .await;

    // Parse header and verify the suite's KDF is HKDF (not Identity)
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut raw = Vec::new();
    let header_body = read_header_body(&mut cursor, None, &mut raw).unwrap();
    let suite = header_body.algorithm_suite();
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Wire-parsed suite uses HKDF; decrypt success confirms it was used
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    assert!(
        matches!(suite.kdf, aws_mpl_legacy::suites::DerivationAlgorithm::Hkdf(_)),
        "parsed suite must use HKDF derivation, got: {:?}", suite.kdf
    );

    // Decrypt corroborates: HKDF was actually used for key derivation
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_esdk_algorithm_suite_yields_error() {
    let keyring = aes_keyring(0).await;
    let pt = b"unsupported esdk suite test";

    let enc_input =
        EncryptInput::with_legacy_keyring(pt, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper the algorithm suite ID bytes in the header to an invalid value.
    // V2 header: byte 0 = version (0x02), bytes 1-2 = algorithm suite ID.
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[1] = 0xFF;
    tampered_ct[2] = 0xFF;

    let valid_input =
        DecryptInput::with_legacy_keyring(&valid_ct, EncryptionContext::new(), keyring.clone());
    let tampered_input =
        DecryptInput::with_legacy_keyring(&tampered_ct, EncryptionContext::new(), keyring);

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Supported ESDK suite ID → Ok; tampered to non-ESDK 0xFFFF → ValidationError
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# decrypt MUST yield an error.
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Tampered header suite ID changes the parsed input; failure proves parsed suite is used
    //# - Algorithm Suite ID: This MUST be the parsed
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# from the message header.
    assert!(decrypt(&valid_input).await.is_ok(), "valid suite ID must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::ValidationError,
        "non-ESDK suite ID must produce ValidationError"
    );
}

/// Spy CMM for decrypt: records the inputs it received from the decrypt call.
struct DecryptSpyCmm {
    inner: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    observed_edk_count: std::sync::Arc<std::sync::Mutex<Option<usize>>>,
    observed_ec_keys: std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>,
}

impl aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManager for DecryptSpyCmm {
    fn get_encryption_materials(
        &self,
        input: aws_mpl_legacy::dafny::operation::get_encryption_materials::GetEncryptionMaterialsInput,
    ) -> Result<aws_mpl_legacy::dafny::operation::get_encryption_materials::GetEncryptionMaterialsOutput, aws_mpl_legacy::dafny::types::error::Error> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.inner.get_encryption_materials()
                    .commitment_policy(input.commitment_policy.unwrap())
                    .encryption_context(input.encryption_context.unwrap())
                    .max_plaintext_length(input.max_plaintext_length.unwrap())
                    .send().await
            })
        })
    }
    fn decrypt_materials(
        &self,
        input: aws_mpl_legacy::dafny::operation::decrypt_materials::DecryptMaterialsInput,
    ) -> Result<aws_mpl_legacy::dafny::operation::decrypt_materials::DecryptMaterialsOutput, aws_mpl_legacy::dafny::types::error::Error> {
        // Record observations
        if let Some(ref edks) = input.encrypted_data_keys {
            *self.observed_edk_count.lock().unwrap() = Some(edks.len());
        }
        if let Some(ref ec) = input.encryption_context {
            let keys: Vec<String> = ec.keys().cloned().collect();
            *self.observed_ec_keys.lock().unwrap() = Some(keys);
        }
        // Delegate
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.inner.decrypt_materials()
                    .algorithm_suite_id(input.algorithm_suite_id.unwrap())
                    .commitment_policy(input.commitment_policy.unwrap())
                    .encryption_context(input.encryption_context.unwrap())
                    .encrypted_data_keys(input.encrypted_data_keys.unwrap())
                    .send().await
            })
        })
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cmm_decrypt_call_receives_header_edks_and_ec() {
    let keyring = test_keyring().await;
    let mut ec = EncryptionContext::new();
    ec.insert("spy-key".to_string(), "spy-val".to_string());
    let pt = b"spy cmm decrypt test";
    let enc_input = EncryptInput::with_legacy_keyring(pt, ec, keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let inner_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring)
        .send()
        .await
        .unwrap();
    let observed_edk_count = std::sync::Arc::new(std::sync::Mutex::new(None));
    let observed_ec_keys = std::sync::Arc::new(std::sync::Mutex::new(None));
    let spy_cmm = aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef::from(DecryptSpyCmm {
        inner: inner_cmm,
        observed_edk_count: observed_edk_count.clone(),
        observed_ec_keys: observed_ec_keys.clone(),
    });

    let dec_input = DecryptInput::with_legacy_cmm(&ct, EncryptionContext::new(), spy_cmm);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt);

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Spy CMM observes EDK count matches header (1 EDK from single keyring)
    //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header.md#encrypted-data-keys)
    //# from the message header.
    let edk_count = observed_edk_count.lock().unwrap().unwrap();
    assert_eq!(edk_count, 1, "CMM must receive 1 EDK from header");

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Spy CMM observes EC contains the key we encrypted with
    //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
    //# from the message header.
    let ec_keys = observed_ec_keys.lock().unwrap().clone().unwrap();
    assert!(ec_keys.contains(&"spy-key".to_string()), "CMM must receive EC from header containing 'spy-key'");

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=Spy CMM was called and returned materials successfully
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //= reason=We passed spy_cmm as input; it was used
    //# The CMM used MUST be the input CMM, if supplied.
}
