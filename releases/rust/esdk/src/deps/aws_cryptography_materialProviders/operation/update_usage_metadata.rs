// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateUsageMetadata`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateUsageMetadata;
impl UpdateUsageMetadata {
    /// Creates a new `UpdateUsageMetadata`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        cryptographic_materials_cache: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::UpdateUsageMetadataInput,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.identifier.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "identifier",
        "identifier was not specified but it is required when building UpdateUsageMetadataInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.bytes_used.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "bytes_used",
        "bytes_used was not specified but it is required when building UpdateUsageMetadataInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if matches!(input.bytes_used, Some(x) if !(0..).contains(&x)) {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "bytes_used",
        "bytes_used failed to satisfy constraint: Member must be greater than or equal to 0",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
        cryptographic_materials_cache.inner.borrow_mut().update_usage_metadata(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::_update_usage_metadata_input::UpdateUsageMetadataInput;

pub(crate) mod _unit;

pub(crate) mod _update_usage_metadata_input;

/// Builders
pub mod builders;
