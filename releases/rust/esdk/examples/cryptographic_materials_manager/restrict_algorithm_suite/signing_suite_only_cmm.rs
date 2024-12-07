// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsInput;
use aws_esdk::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput;
use aws_esdk::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsInput;
use aws_esdk::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput;
use aws_esdk::aws_cryptography_materialProviders::types::error::Error;
use aws_esdk::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManager;
use aws_esdk::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_esdk::aws_cryptography_materialProviders::types::keyring::KeyringRef;
use aws_esdk::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId;
use aws_esdk::aws_cryptography_materialProviders::types::AlgorithmSuiteId;
use aws_esdk::aws_cryptography_materialProviders::client as mpl_client;
use aws_esdk::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig;
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
        let mpl_config = MaterialProvidersConfig::builder().build().unwrap();
        let mpl = mpl_client::Client::from_conf(mpl_config).unwrap();

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
                    mpl.create_default_cryptographic_materials_manager()
                        .keyring(keyring)
                        .send()
                        .await
                })
            }).unwrap(),
        }
    }
}

impl CryptographicMaterialsManager for SigningSuiteOnlyCMM {
    fn get_encryption_materials(
        &self,
        input: GetEncryptionMaterialsInput,
    ) -> Result<GetEncryptionMaterialsOutput, Error> {
        let algorithm_suite_id: AlgorithmSuiteId = input.algorithm_suite_id.clone().unwrap();
        let esdk_algorithm_suite_id: EsdkAlgorithmSuiteId = if let AlgorithmSuiteId::Esdk(esdk_id) = algorithm_suite_id {
            esdk_id
        }
        else {
            panic!("Algorithm Suite ID is not an EsdkAlgorithmSuiteId");
        };

        if !self.approved_algos.contains(&esdk_algorithm_suite_id) {
            return Err(Error::AwsCryptographicMaterialProvidersException {
                message: "Algorithm Suite must use Signing".to_string(),
            });
        }

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.cmm.get_encryption_materials()
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
        let esdk_algorithm_suite_id: EsdkAlgorithmSuiteId = if let AlgorithmSuiteId::Esdk(esdk_id) = algorithm_suite_id {
            esdk_id
        }
        else {
            panic!("Algorithm Suite ID is not an EsdkAlgorithmSuiteId");
        };

        if !self.approved_algos.contains(&esdk_algorithm_suite_id) {
            return Err(Error::AwsCryptographicMaterialProvidersException {
                message: "Algorithm Suite must use Signing".to_string(),
            });
        }

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.cmm.decrypt_materials()
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
