// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetAlgorithmSuiteInfoInput {
    #[allow(missing_docs)]
    pub binary_id: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetAlgorithmSuiteInfoInput {
    #[allow(missing_docs)]
    pub fn binary_id(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.binary_id
    }
}
impl GetAlgorithmSuiteInfoInput {
    /// Creates a new builder-style object to manufacture [`GetAlgorithmSuiteInfoInput`](crate::operation::get_algorithm_suite_info::builders::GetAlgorithmSuiteInfoInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::builders::GetAlgorithmSuiteInfoInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::builders::GetAlgorithmSuiteInfoInputBuilder::default()
    }
}

/// A builder for [`GetAlgorithmSuiteInfoInput`](crate::operation::operation::GetAlgorithmSuiteInfoInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAlgorithmSuiteInfoInputBuilder {
    pub(crate) binary_id: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetAlgorithmSuiteInfoInputBuilder {
    #[allow(missing_docs)]
    pub fn binary_id(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.binary_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_binary_id(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.binary_id = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_binary_id(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.binary_id
    }
    /// Consumes the builder and constructs a [`GetAlgorithmSuiteInfoInput`](crate::operation::operation::GetAlgorithmSuiteInfoInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput {
            binary_id: self.binary_id,
        })
    }
}
