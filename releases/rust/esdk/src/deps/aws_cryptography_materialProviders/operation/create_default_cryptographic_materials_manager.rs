// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateDefaultCryptographicMaterialsManager`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateDefaultCryptographicMaterialsManager;
impl CreateDefaultCryptographicMaterialsManager {
    /// Creates a new `CreateDefaultCryptographicMaterialsManager`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput_for_AwsCryptographicMaterialProviders_CreateDefaultCryptographicMaterialsManager(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::create_default_cryptographic_materials_manager::_create_default_cryptographic_materials_manager_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(client.dafny_client.clone())
            .CreateDefaultCryptographicMaterialsManager(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_manager::from_dafny(inner_result.value().clone())
,
            )
        } else {
            Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::_create_cryptographic_materials_manager_output::CreateCryptographicMaterialsManagerOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::_create_default_cryptographic_materials_manager_input::CreateDefaultCryptographicMaterialsManagerInput;

pub(crate) mod _create_cryptographic_materials_manager_output;

pub(crate) mod _create_default_cryptographic_materials_manager_input;

/// Builders
pub mod builders;
