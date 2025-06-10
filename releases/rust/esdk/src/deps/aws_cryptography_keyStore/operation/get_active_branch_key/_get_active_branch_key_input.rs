// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for getting a Branch Key's ACTIVE version.
pub struct GetActiveBranchKeyInput {
    /// The identifier for the Branch Key to get the ACTIVE version for.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl GetActiveBranchKeyInput {
    /// The identifier for the Branch Key to get the ACTIVE version for.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
}
impl GetActiveBranchKeyInput {
    /// Creates a new builder-style object to manufacture [`GetActiveBranchKeyInput`](crate::operation::get_active_branch_key::builders::GetActiveBranchKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::builders::GetActiveBranchKeyInputBuilder{
        crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::builders::GetActiveBranchKeyInputBuilder::default()
    }
}

/// A builder for [`GetActiveBranchKeyInput`](crate::operation::operation::GetActiveBranchKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetActiveBranchKeyInputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl GetActiveBranchKeyInputBuilder {
    /// The identifier for the Branch Key to get the ACTIVE version for.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the Branch Key to get the ACTIVE version for.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// The identifier for the Branch Key to get the ACTIVE version for.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Consumes the builder and constructs a [`GetActiveBranchKeyInput`](crate::operation::operation::GetActiveBranchKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::GetActiveBranchKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::GetActiveBranchKeyInput {
            branch_key_identifier: self.branch_key_identifier,
        })
    }
}
