// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for for creating an AWS KMS MRK Keyring.
pub struct CreateAwsKmsMrkKeyringInput {
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The KMS Client this Keyring will use to call KMS.
    pub kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    /// The identifier for the symmetric AWS KMS Key or AWS KMS Multi-Region Key responsible for wrapping and unwrapping data keys.
    pub kms_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateAwsKmsMrkKeyringInput {
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn kms_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.kms_client
    }
    /// The identifier for the symmetric AWS KMS Key or AWS KMS Multi-Region Key responsible for wrapping and unwrapping data keys.
    pub fn kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
}
impl CreateAwsKmsMrkKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsMrkKeyringInput`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsMrkKeyringInput`](crate::operation::operation::CreateAwsKmsMrkKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsMrkKeyringInputBuilder {
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateAwsKmsMrkKeyringInputBuilder {
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = ::std::option::Option::Some(input.into());
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn kms_client(
        mut self,
        input: impl ::std::convert::Into<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.kms_client = ::std::option::Option::Some(input.into());
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn set_kms_client(
        mut self,
        input: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.kms_client = input;
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn get_kms_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.kms_client
    }
    /// The identifier for the symmetric AWS KMS Key or AWS KMS Multi-Region Key responsible for wrapping and unwrapping data keys.
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the symmetric AWS KMS Key or AWS KMS Multi-Region Key responsible for wrapping and unwrapping data keys.
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// The identifier for the symmetric AWS KMS Key or AWS KMS Multi-Region Key responsible for wrapping and unwrapping data keys.
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsMrkKeyringInput`](crate::operation::operation::CreateAwsKmsMrkKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_keyring::CreateAwsKmsMrkKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_keyring::CreateAwsKmsMrkKeyringInput {
            grant_tokens: self.grant_tokens,
kms_client: self.kms_client,
kms_key_id: self.kms_key_id,
        })
    }
}
