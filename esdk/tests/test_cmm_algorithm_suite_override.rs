// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Test: the algorithm suite in the retrieved encryption materials MAY be
//! different from the input algorithm suite (encrypt.md#get-the-encryption-materials).

mod fixtures;
mod test_helpers;
use aws_esdk::*;
use aws_mpl_legacy::dafny::operation::decrypt_materials::{
    DecryptMaterialsInput, DecryptMaterialsOutput,
};
use aws_mpl_legacy::dafny::operation::get_encryption_materials::{
    GetEncryptionMaterialsInput, GetEncryptionMaterialsOutput,
};
use aws_mpl_legacy::dafny::types::cryptographic_materials_manager::{
    CryptographicMaterialsManager, CryptographicMaterialsManagerRef,
};
use aws_mpl_legacy::dafny::types::error::Error as MplError;
use aws_mpl_legacy::dafny::types::{AlgorithmSuiteId, EsdkAlgorithmSuiteId};
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as SuiteId;
use fixtures::*;

/// Wraps a real CMM but forces a different algorithm suite on encrypt.
struct SuiteOverrideCmm {
    inner: CryptographicMaterialsManagerRef,
    suite: EsdkAlgorithmSuiteId,
}

impl CryptographicMaterialsManager for SuiteOverrideCmm {
    fn get_encryption_materials(
        &self,
        input: GetEncryptionMaterialsInput,
    ) -> Result<GetEncryptionMaterialsOutput, MplError> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.inner
                    .get_encryption_materials()
                    .algorithm_suite_id(AlgorithmSuiteId::Esdk(self.suite.clone()))
                    .commitment_policy(input.commitment_policy.unwrap())
                    .encryption_context(input.encryption_context.unwrap())
                    .max_plaintext_length(input.max_plaintext_length.unwrap())
                    .send()
                    .await
            })
        })
    }
    fn decrypt_materials(
        &self,
        input: DecryptMaterialsInput,
    ) -> Result<DecryptMaterialsOutput, MplError> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.inner
                    .decrypt_materials()
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
async fn test_encrypt_uses_cmm_suite_not_input_suite() {
    let (ns, name) = namespace_and_name(0);
    let keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();

    let cmm = CryptographicMaterialsManagerRef::from(SuiteOverrideCmm {
        inner: mpl()
            .create_default_cryptographic_materials_manager()
            .keyring(keyring.clone())
            .send()
            .await
            .unwrap(),
        suite: EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
    });

    // Caller requests signing+committing, but CMM overrides to committing-only.
    let mut enc_input = EncryptInput::with_legacy_cmm(b"hello", EncryptionContext::new(), cmm);
    enc_input.algorithm_suite_id = Some(SuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);

    let out = encrypt(&enc_input).await.unwrap();

    // Output must reflect the CMM's suite, not the caller's.
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# Note that the algorithm suite in the retrieved encryption materials MAY be different
    //# from the [input algorithm suite](#algorithm-suite).
    assert_eq!(
        out.algorithm_suite_id,
        SuiteId::AlgAes256GcmHkdfSha512CommitKey
    );
    assert_ne!(
        out.algorithm_suite_id,
        SuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
    );

    // Round-trip to prove the ciphertext is valid.
    let dec = decrypt(&DecryptInput::with_legacy_keyring(
        &out.ciphertext,
        EncryptionContext::new(),
        keyring,
    ))
    .await
    .unwrap();
    assert_eq!(dec.plaintext, b"hello");
    assert_eq!(
        dec.algorithm_suite_id,
        SuiteId::AlgAes256GcmHkdfSha512CommitKey
    );
}
