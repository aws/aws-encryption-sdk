// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DecryptionMaterialsWithPlaintextDataKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecryptionMaterialsWithPlaintextDataKey;
impl DecryptionMaterialsWithPlaintextDataKey {
    /// Creates a new `DecryptionMaterialsWithPlaintextDataKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterials,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_for_AwsCryptographicMaterialProviders_DecryptionMaterialsWithPlaintextDataKey(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials_with_plaintext_data_key::_decryption_materials_with_plaintext_data_key_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone())
            .DecryptionMaterialsWithPlaintextDataKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(())
        } else {
            Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::_decryption_materials::DecryptionMaterials;

pub(crate) mod _unit;

pub(crate) mod _decryption_materials;

/// Builders
pub mod builders;
