// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::mpl;
use aws_mpl_legacy::operation::decrypt_materials::DecryptMaterialsInput;
use aws_mpl_legacy::operation::decrypt_materials::DecryptMaterialsOutput;
use aws_mpl_legacy::operation::get_encryption_materials::GetEncryptionMaterialsInput;
use aws_mpl_legacy::operation::get_encryption_materials::GetEncryptionMaterialsOutput;
use aws_mpl_legacy::types::AlgorithmSuiteId;
use aws_mpl_legacy::types::EsdkAlgorithmSuiteId;
use aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManager;
use aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_mpl_legacy::types::error::Error;
use aws_mpl_legacy::types::keyring::KeyringRef;
use std::vec::Vec;

/*
 Demonstrates creating a custom Cryptographic Materials Manager (CMM).
 The SigningSuiteOnlyCMM ensures that callers use an Algorithm Suite with
 signing. This is a best practice. Read more about Digital Signing:
 https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#digital-sigs
 Read more about Cryptographic Materials Managers (CMMs):
 https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#crypt-materials-manager
*/

pub struct SigningSuiteOnlyCMM {
    approved_algos: Vec<EsdkAlgorithmSuiteId>,
    cmm: CryptographicMaterialsManagerRef,
}

impl SigningSuiteOnlyCMM {
    pub fn new(keyring: KeyringRef) -> Self {
        Self {
            approved_algos: vec![
                EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
                EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
                EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
                EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
            ],
            // Create a DefaultCryptographicMaterialsManager to facilitate
            // GetEncryptionMaterials and DecryptionMaterials
            // after this CMM approves the Algorithm Suite.
            cmm: tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    mpl()
                        .create_default_cryptographic_materials_manager()
                        .keyring(keyring)
                        .send()
                        .await
                })
            })
            .unwrap(),
        }
    }
}

impl CryptographicMaterialsManager for SigningSuiteOnlyCMM {
    fn get_encryption_materials(
        &self,
        input: GetEncryptionMaterialsInput,
    ) -> Result<GetEncryptionMaterialsOutput, Error> {
        let algorithm_suite_id: AlgorithmSuiteId = input.algorithm_suite_id.clone().unwrap();
        let esdk_algorithm_suite_id: EsdkAlgorithmSuiteId =
            if let AlgorithmSuiteId::Esdk(esdk_id) = algorithm_suite_id {
                esdk_id
            } else {
                panic!("Algorithm Suite ID is not an EsdkAlgorithmSuiteId");
            };

        if !self.approved_algos.contains(&esdk_algorithm_suite_id) {
            return Err(Error::AwsCryptographicMaterialProvidersException {
                message: "Algorithm Suite must use Signing".to_string(),
            });
        }

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.cmm
                    .get_encryption_materials()
                    .algorithm_suite_id(input.algorithm_suite_id.unwrap())
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
    ) -> Result<DecryptMaterialsOutput, Error> {
        let algorithm_suite_id: AlgorithmSuiteId = input.algorithm_suite_id.clone().unwrap();
        let esdk_algorithm_suite_id: EsdkAlgorithmSuiteId =
            if let AlgorithmSuiteId::Esdk(esdk_id) = algorithm_suite_id {
                esdk_id
            } else {
                panic!("Algorithm Suite ID is not an EsdkAlgorithmSuiteId");
            };

        if !self.approved_algos.contains(&esdk_algorithm_suite_id) {
            return Err(Error::AwsCryptographicMaterialProvidersException {
                message: "Algorithm Suite must use Signing".to_string(),
            });
        }

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.cmm
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
