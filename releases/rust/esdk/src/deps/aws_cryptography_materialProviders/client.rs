// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
#[allow(missing_docs)]
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IAwsCryptographicMaterialProvidersClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        input: crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig,
    ) -> Result<Self, crate::deps::aws_cryptography_materialProviders::types::error::Error> {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HMaterialProvidersConfig(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner =
            crate::software::amazon::cryptography::materialproviders::internaldafny::_default::MaterialProviders(
                &crate::deps::aws_cryptography_materialProviders::conversions::material_providers_config::_material_providers_config::to_dafny(input),
            );
        if matches!(
            inner.as_ref(),
            crate::_Wrappers_Compile::Result::Failure { .. }
        ) {
            return Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner.as_ref().error().clone(),
                ),
            );
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract()),
        })
    }
}

mod create_aws_kms_keyring;

mod create_aws_kms_discovery_keyring;

mod create_aws_kms_multi_keyring;

mod create_aws_kms_discovery_multi_keyring;

mod create_aws_kms_mrk_keyring;

mod create_aws_kms_mrk_multi_keyring;

mod create_aws_kms_mrk_discovery_keyring;

mod create_aws_kms_mrk_discovery_multi_keyring;

mod create_aws_kms_hierarchical_keyring;

mod create_aws_kms_rsa_keyring;

mod create_aws_kms_ecdh_keyring;

mod create_multi_keyring;

mod create_raw_aes_keyring;

mod create_raw_rsa_keyring;

mod create_raw_ecdh_keyring;

mod create_default_cryptographic_materials_manager;

mod create_required_encryption_context_cmm;

mod create_cryptographic_materials_cache;

mod create_default_client_supplier;

mod initialize_encryption_materials;

mod initialize_decryption_materials;

mod valid_encryption_materials_transition;

mod valid_decryption_materials_transition;

mod encryption_materials_has_plaintext_data_key;

mod decryption_materials_with_plaintext_data_key;

mod get_algorithm_suite_info;

mod valid_algorithm_suite_info;

mod validate_commitment_policy_on_encrypt;

mod validate_commitment_policy_on_decrypt;
