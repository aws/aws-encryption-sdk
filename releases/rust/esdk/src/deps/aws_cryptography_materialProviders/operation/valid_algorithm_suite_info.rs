// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ValidAlgorithmSuiteInfo`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ValidAlgorithmSuiteInfo;
impl ValidAlgorithmSuiteInfo {
    /// Creates a new `ValidAlgorithmSuiteInfo`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.id.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "id",
        "id was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.binary_id.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "binary_id",
        "binary_id was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.message_version.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "message_version",
        "message_version was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.encrypt.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encrypt",
        "encrypt was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.kdf.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "kdf",
        "kdf was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.commitment.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "commitment",
        "commitment was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.signature.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "signature",
        "signature was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.symmetric_signature.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "symmetric_signature",
        "symmetric_signature was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
if input.edk_wrapping.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "edk_wrapping",
        "edk_wrapping was not specified but it is required when building AlgorithmSuiteInfo",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
                let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::valid_algorithm_suite_info::_valid_algorithm_suite_info_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ValidAlgorithmSuiteInfo(&inner_input);
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

pub use crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::_algorithm_suite_info::AlgorithmSuiteInfo;

pub(crate) mod _unit;

pub(crate) mod _algorithm_suite_info;

/// Builders
pub mod builders;
