// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecryptOutput {
    #[allow(missing_docs)]
pub algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
#[allow(missing_docs)]
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)]
pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptOutput {
    #[allow(missing_docs)]
pub fn algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    &self.algorithm_suite_id
}
#[allow(missing_docs)]
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)]
pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
}
impl DecryptOutput {
    /// Creates a new builder-style object to manufacture [`DecryptOutput`](crate::operation::decrypt::builders::DecryptOutput).
    pub fn builder() -> crate::operation::decrypt::builders::DecryptOutputBuilder {
        crate::operation::decrypt::builders::DecryptOutputBuilder::default()
    }
}

/// A builder for [`DecryptOutput`](crate::operation::operation::DecryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptOutputBuilder {
    pub(crate) algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptOutputBuilder {
    #[allow(missing_docs)]
pub fn algorithm_suite_id(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.algorithm_suite_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_algorithm_suite_id(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.algorithm_suite_id = input;
    self
}
#[allow(missing_docs)]
pub fn get_algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    &self.algorithm_suite_id
}
#[allow(missing_docs)]
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = input;
    self
}
#[allow(missing_docs)]
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)]
pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = input;
    self
}
#[allow(missing_docs)]
pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
    /// Consumes the builder and constructs a [`DecryptOutput`](crate::operation::operation::DecryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::decrypt::DecryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::decrypt::DecryptOutput {
            algorithm_suite_id: self.algorithm_suite_id,
encryption_context: self.encryption_context,
plaintext: self.plaintext,
        })
    }
}
