// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ValidDecryptionMaterialsTransition`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ValidDecryptionMaterialsTransition;
impl ValidDecryptionMaterialsTransition {
    /// Creates a new `ValidDecryptionMaterialsTransition`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_for_AwsCryptographicMaterialProviders_ValidDecryptionMaterialsTransition(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::valid_decryption_materials_transition::_valid_decryption_materials_transition_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone())
            .ValidDecryptionMaterialsTransition(&inner_input);
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

pub use crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::_valid_decryption_materials_transition_input::ValidDecryptionMaterialsTransitionInput;

pub(crate) mod _unit;

pub(crate) mod _valid_decryption_materials_transition_input;

/// Builders
pub mod builders;
