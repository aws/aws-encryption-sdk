// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for for creating a AWS KMS Discovery Keyring.
pub struct CreateAwsKmsDiscoveryKeyringInput {
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The KMS Client this Keyring will use to call KMS.
    pub kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl CreateAwsKmsDiscoveryKeyringInput {
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn discovery_filter(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    > {
        &self.discovery_filter
    }
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
}
impl CreateAwsKmsDiscoveryKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsDiscoveryKeyringInput`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsDiscoveryKeyringInput`](crate::operation::operation::CreateAwsKmsDiscoveryKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsDiscoveryKeyringInputBuilder {
    pub(crate) discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl CreateAwsKmsDiscoveryKeyringInputBuilder {
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn discovery_filter(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.discovery_filter = ::std::option::Option::Some(input.into());
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn set_discovery_filter(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.discovery_filter = input;
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn get_discovery_filter(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    > {
        &self.discovery_filter
    }
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
    /// Consumes the builder and constructs a [`CreateAwsKmsDiscoveryKeyringInput`](crate::operation::operation::CreateAwsKmsDiscoveryKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput {
            discovery_filter: self.discovery_filter,
grant_tokens: self.grant_tokens,
kms_client: self.kms_client,
        })
    }
}
