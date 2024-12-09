// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EncryptionMaterialsHasPlaintextDataKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EncryptionMaterialsHasPlaintextDataKey;
impl EncryptionMaterialsHasPlaintextDataKey {
    /// Creates a new `EncryptionMaterialsHasPlaintextDataKey`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::encryption_materials_has_plaintext_data_key::EncryptionMaterials,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.algorithm_suite.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "algorithm_suite",
        "algorithm_suite was not specified but it is required when building EncryptionMaterials",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encryption_context.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encryption_context",
        "encryption_context was not specified but it is required when building EncryptionMaterials",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encrypted_data_keys.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encrypted_data_keys",
        "encrypted_data_keys was not specified but it is required when building EncryptionMaterials",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.required_encryption_context_keys.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "required_encryption_context_keys",
        "required_encryption_context_keys was not specified but it is required when building EncryptionMaterials",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::encryption_materials_has_plaintext_data_key::_encryption_materials_has_plaintext_data_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).EncryptionMaterialsHasPlaintextDataKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                (),
            )
        } else {
            Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::encryption_materials_has_plaintext_data_key::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::encryption_materials_has_plaintext_data_key::_encryption_materials::EncryptionMaterials;

pub(crate) mod _unit;

pub(crate) mod _encryption_materials;

/// Builders
pub mod builders;
