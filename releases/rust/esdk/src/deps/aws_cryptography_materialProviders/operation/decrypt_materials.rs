// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DecryptMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecryptMaterials;
impl DecryptMaterials {
    /// Creates a new `DecryptMaterials`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        cryptographic_materials_manager: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.algorithm_suite_id.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "algorithm_suite_id",
        "algorithm_suite_id was not specified but it is required when building DecryptMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.commitment_policy.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "commitment_policy",
        "commitment_policy was not specified but it is required when building DecryptMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encrypted_data_keys.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encrypted_data_keys",
        "encrypted_data_keys was not specified but it is required when building DecryptMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encryption_context.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encryption_context",
        "encryption_context was not specified but it is required when building DecryptMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
        cryptographic_materials_manager.inner.borrow_mut().decrypt_materials(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::_decrypt_materials_output::DecryptMaterialsOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::_decrypt_materials_input::DecryptMaterialsInput;

pub(crate) mod _decrypt_materials_output;

pub(crate) mod _decrypt_materials_input;

/// Builders
pub mod builders;
