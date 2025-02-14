// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for Key Store DynamoDB table creation.
pub struct CreateKeyStoreOutput {
    /// The ARN of the DynamoDB table that backs this Key Store.
    pub table_arn: ::std::option::Option<::std::string::String>,
}
impl CreateKeyStoreOutput {
    /// The ARN of the DynamoDB table that backs this Key Store.
    pub fn table_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_arn
    }
}
impl CreateKeyStoreOutput {
    /// Creates a new builder-style object to manufacture [`CreateKeyStoreOutput`](crate::operation::create_key_store::builders::CreateKeyStoreOutput).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreOutputBuilder{
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreOutputBuilder::default()
    }
}

/// A builder for [`CreateKeyStoreOutput`](crate::operation::operation::CreateKeyStoreOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyStoreOutputBuilder {
    pub(crate) table_arn: ::std::option::Option<::std::string::String>,
}
impl CreateKeyStoreOutputBuilder {
    /// The ARN of the DynamoDB table that backs this Key Store.
    pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the DynamoDB table that backs this Key Store.
    pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_arn = input;
        self
    }
    /// The ARN of the DynamoDB table that backs this Key Store.
    pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_arn
    }
    /// Consumes the builder and constructs a [`CreateKeyStoreOutput`](crate::operation::operation::CreateKeyStoreOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput {
            table_arn: self.table_arn,
        })
    }
}
