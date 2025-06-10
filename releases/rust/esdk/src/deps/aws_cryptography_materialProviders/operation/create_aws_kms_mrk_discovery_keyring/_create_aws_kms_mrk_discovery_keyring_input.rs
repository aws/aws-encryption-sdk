// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for for creating a AWS KMS MRK Discovery Keyring.
pub struct CreateAwsKmsMrkDiscoveryKeyringInput {
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The KMS Client this Keyring will use to call KMS.
    pub kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    /// The region the input 'kmsClient' is in.
    pub region: ::std::option::Option<::std::string::String>,
}
impl CreateAwsKmsMrkDiscoveryKeyringInput {
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
    /// The region the input 'kmsClient' is in.
    pub fn region(&self) -> &::std::option::Option<::std::string::String> {
        &self.region
    }
}
impl CreateAwsKmsMrkDiscoveryKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsMrkDiscoveryKeyringInput`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsMrkDiscoveryKeyringInput`](crate::operation::operation::CreateAwsKmsMrkDiscoveryKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsMrkDiscoveryKeyringInputBuilder {
    pub(crate) discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
}
impl CreateAwsKmsMrkDiscoveryKeyringInputBuilder {
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
    /// The region the input 'kmsClient' is in.
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// The region the input 'kmsClient' is in.
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// The region the input 'kmsClient' is in.
    pub fn get_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.region
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsMrkDiscoveryKeyringInput`](crate::operation::operation::CreateAwsKmsMrkDiscoveryKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::CreateAwsKmsMrkDiscoveryKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::CreateAwsKmsMrkDiscoveryKeyringInput {
            discovery_filter: self.discovery_filter,
grant_tokens: self.grant_tokens,
kms_client: self.kms_client,
region: self.region,
        })
    }
}
