// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetAlgorithmSuiteInfo`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetAlgorithmSuiteInfo;
impl GetAlgorithmSuiteInfo {
    /// Creates a new `GetAlgorithmSuiteInfo`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
        input: crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::AlgorithmSuiteInfo,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput_for_AwsCryptographicMaterialProviders_GetAlgorithmSuiteInfo(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        let inner_input =
            crate::standard_library_conversions::blob_to_dafny(&input.r#binary_id.unwrap());
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetAlgorithmSuiteInfo(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::deps::aws_cryptography_materialProviders::conversions::get_algorithm_suite_info::_get_algorithm_suite_info_output::from_dafny(inner_result.value().clone()),
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

pub use crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::_algorithm_suite_info::AlgorithmSuiteInfo;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::_get_algorithm_suite_info_input::GetAlgorithmSuiteInfoInput;

pub(crate) mod _algorithm_suite_info;

pub(crate) mod _get_algorithm_suite_info_input;

/// Builders
pub mod builders;
