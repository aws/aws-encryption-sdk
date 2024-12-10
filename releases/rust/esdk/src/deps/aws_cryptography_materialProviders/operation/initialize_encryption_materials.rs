// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `InitializeEncryptionMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InitializeEncryptionMaterials;
impl InitializeEncryptionMaterials {
    /// Creates a new `InitializeEncryptionMaterials`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::InitializeEncryptionMaterialsInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.algorithm_suite_id.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "algorithm_suite_id",
        "algorithm_suite_id was not specified but it is required when building InitializeEncryptionMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encryption_context.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encryption_context",
        "encryption_context was not specified but it is required when building InitializeEncryptionMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.required_encryption_context_keys.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "required_encryption_context_keys",
        "required_encryption_context_keys was not specified but it is required when building InitializeEncryptionMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::initialize_encryption_materials::_initialize_encryption_materials_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).InitializeEncryptionMaterials(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_materialProviders::conversions::initialize_encryption_materials::_initialize_encryption_materials_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::_encryption_materials::EncryptionMaterials;

pub use crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::_initialize_encryption_materials_input::InitializeEncryptionMaterialsInput;

pub(crate) mod _encryption_materials;

pub(crate) mod _initialize_encryption_materials_input;

/// Builders
pub mod builders;
