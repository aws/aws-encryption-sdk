// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for getting a version of a Branch Key.
pub struct GetBranchKeyVersionInput {
    /// The identifier for the Branch Key to get a particular version for.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
    /// The version to get.
    pub branch_key_version: ::std::option::Option<::std::string::String>,
}
impl GetBranchKeyVersionInput {
    /// The identifier for the Branch Key to get a particular version for.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// The version to get.
    pub fn branch_key_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_version
    }
}
impl GetBranchKeyVersionInput {
    /// Creates a new builder-style object to manufacture [`GetBranchKeyVersionInput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::GetBranchKeyVersionInputBuilder
    {
        crate::deps::aws_cryptography_keyStore::types::builders::GetBranchKeyVersionInputBuilder::default()
    }
}

/// A builder for [`GetBranchKeyVersionInput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBranchKeyVersionInputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) branch_key_version: ::std::option::Option<::std::string::String>,
}
impl GetBranchKeyVersionInputBuilder {
    /// The identifier for the Branch Key to get a particular version for.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the Branch Key to get a particular version for.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// The identifier for the Branch Key to get a particular version for.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// The version to get.
    pub fn branch_key_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_version = ::std::option::Option::Some(input.into());
        self
    }
    /// The version to get.
    pub fn set_branch_key_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_version = input;
        self
    }
    /// The version to get.
    pub fn get_branch_key_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_version
    }
    /// Consumes the builder and constructs a [`GetBranchKeyVersionInput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput {
                branch_key_identifier: self.branch_key_identifier,
                branch_key_version: self.branch_key_version,
            },
        )
    }
}
