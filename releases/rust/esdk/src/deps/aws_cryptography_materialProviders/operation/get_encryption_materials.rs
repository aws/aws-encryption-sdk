// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetEncryptionMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetEncryptionMaterials;
impl GetEncryptionMaterials {
    /// Creates a new `GetEncryptionMaterials`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        cryptographic_materials_manager: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.encryption_context.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encryption_context",
        "encryption_context was not specified but it is required when building GetEncryptionMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.commitment_policy.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "commitment_policy",
        "commitment_policy was not specified but it is required when building GetEncryptionMaterialsInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
        cryptographic_materials_manager.inner.borrow_mut().get_encryption_materials(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::_get_encryption_materials_output::GetEncryptionMaterialsOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::_get_encryption_materials_input::GetEncryptionMaterialsInput;

pub(crate) mod _get_encryption_materials_output;

pub(crate) mod _get_encryption_materials_input;

/// Builders
pub mod builders;
