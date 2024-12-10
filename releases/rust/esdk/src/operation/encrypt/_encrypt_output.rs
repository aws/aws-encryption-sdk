// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EncryptOutput {
    #[allow(missing_docs)]
pub algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
#[allow(missing_docs)]
pub ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)]
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl EncryptOutput {
    #[allow(missing_docs)]
pub fn algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    &self.algorithm_suite_id
}
#[allow(missing_docs)]
pub fn ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext
}
#[allow(missing_docs)]
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
}
impl EncryptOutput {
    /// Creates a new builder-style object to manufacture [`EncryptOutput`](crate::operation::encrypt::builders::EncryptOutput).
    pub fn builder() -> crate::operation::encrypt::builders::EncryptOutputBuilder {
        crate::operation::encrypt::builders::EncryptOutputBuilder::default()
    }
}

/// A builder for [`EncryptOutput`](crate::operation::operation::EncryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptOutputBuilder {
    pub(crate) algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
pub(crate) ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl EncryptOutputBuilder {
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
pub fn ciphertext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_ciphertext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext = input;
    self
}
#[allow(missing_docs)]
pub fn get_ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext
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
    /// Consumes the builder and constructs a [`EncryptOutput`](crate::operation::operation::EncryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt::EncryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::encrypt::EncryptOutput {
            algorithm_suite_id: self.algorithm_suite_id,
ciphertext: self.ciphertext,
encryption_context: self.encryption_context,
        })
    }
}
