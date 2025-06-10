// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// A filter which defines what AWS partition and AWS accounts a KMS Key may be in for a Keyring to be allowed to attempt to decrypt it.
pub struct DiscoveryFilter {
    /// A list of allowed AWS account IDs.
    pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The AWS partition which is allowed.
    pub partition: ::std::option::Option<::std::string::String>,
}
impl DiscoveryFilter {
    /// A list of allowed AWS account IDs.
    pub fn account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.account_ids
    }
    /// The AWS partition which is allowed.
    pub fn partition(&self) -> &::std::option::Option<::std::string::String> {
        &self.partition
    }
}
impl DiscoveryFilter {
    /// Creates a new builder-style object to manufacture [`DiscoveryFilter`](crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::DiscoveryFilterBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::DiscoveryFilterBuilder::default()
    }
}

/// A builder for [`DiscoveryFilter`](crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DiscoveryFilterBuilder {
    pub(crate) account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) partition: ::std::option::Option<::std::string::String>,
}
impl DiscoveryFilterBuilder {
    /// A list of allowed AWS account IDs.
    pub fn account_ids(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.account_ids = ::std::option::Option::Some(input.into());
        self
    }
    /// A list of allowed AWS account IDs.
    pub fn set_account_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.account_ids = input;
        self
    }
    /// A list of allowed AWS account IDs.
    pub fn get_account_ids(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.account_ids
    }
    /// The AWS partition which is allowed.
    pub fn partition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.partition = ::std::option::Option::Some(input.into());
        self
    }
    /// The AWS partition which is allowed.
    pub fn set_partition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.partition = input;
        self
    }
    /// The AWS partition which is allowed.
    pub fn get_partition(&self) -> &::std::option::Option<::std::string::String> {
        &self.partition
    }
    /// Consumes the builder and constructs a [`DiscoveryFilter`](crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter {
                account_ids: self.account_ids,
                partition: self.partition,
            },
        )
    }
}
