// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// The configuration information for a Key Store.
pub struct GetKeyStoreInfoOutput {
    /// The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// An identifier for this Key Store.
    pub key_store_id: ::std::option::Option<::std::string::String>,
    /// The DynamoDB table name that backs this Key Store.
    pub key_store_name: ::std::option::Option<::std::string::String>,
    /// Configures Key Store's KMS Key ARN restrictions.
    pub kms_configuration:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>,
    /// The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub logical_key_store_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyStoreInfoOutput {
    /// The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// An identifier for this Key Store.
    pub fn key_store_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_store_id
    }
    /// The DynamoDB table name that backs this Key Store.
    pub fn key_store_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_store_name
    }
    /// Configures Key Store's KMS Key ARN restrictions.
    pub fn kms_configuration(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>
    {
        &self.kms_configuration
    }
    /// The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub fn logical_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.logical_key_store_name
    }
}
impl GetKeyStoreInfoOutput {
    /// Creates a new builder-style object to manufacture [`GetKeyStoreInfoOutput`](crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::GetKeyStoreInfoOutputBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::GetKeyStoreInfoOutputBuilder::default()
    }
}

/// A builder for [`GetKeyStoreInfoOutput`](crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetKeyStoreInfoOutputBuilder {
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) key_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) key_store_name: ::std::option::Option<::std::string::String>,
    pub(crate) kms_configuration:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>,
    pub(crate) logical_key_store_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyStoreInfoOutputBuilder {
    /// The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = ::std::option::Option::Some(input.into());
        self
    }
    /// The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// An identifier for this Key Store.
    pub fn key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// An identifier for this Key Store.
    pub fn set_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_store_id = input;
        self
    }
    /// An identifier for this Key Store.
    pub fn get_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_store_id
    }
    /// The DynamoDB table name that backs this Key Store.
    pub fn key_store_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.key_store_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The DynamoDB table name that backs this Key Store.
    pub fn set_key_store_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.key_store_name = input;
        self
    }
    /// The DynamoDB table name that backs this Key Store.
    pub fn get_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_store_name
    }
    /// Configures Key Store's KMS Key ARN restrictions.
    pub fn kms_configuration(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_keyStore::types::KmsConfiguration,
        >,
    ) -> Self {
        self.kms_configuration = ::std::option::Option::Some(input.into());
        self
    }
    /// Configures Key Store's KMS Key ARN restrictions.
    pub fn set_kms_configuration(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_keyStore::types::KmsConfiguration,
        >,
    ) -> Self {
        self.kms_configuration = input;
        self
    }
    /// Configures Key Store's KMS Key ARN restrictions.
    pub fn get_kms_configuration(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>
    {
        &self.kms_configuration
    }
    /// The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub fn logical_key_store_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.logical_key_store_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub fn set_logical_key_store_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.logical_key_store_name = input;
        self
    }
    /// The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub fn get_logical_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.logical_key_store_name
    }
    /// Consumes the builder and constructs a [`GetKeyStoreInfoOutput`](crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput {
                grant_tokens: self.grant_tokens,
                key_store_id: self.key_store_id,
                key_store_name: self.key_store_name,
                kms_configuration: self.kms_configuration,
                logical_key_store_name: self.logical_key_store_name,
            },
        )
    }
}
