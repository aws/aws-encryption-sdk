// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for versioning a Branch Key.
pub struct VersionKeyInput {
    /// The identifier for the Branch Key to be versioned.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl VersionKeyInput {
    /// The identifier for the Branch Key to be versioned.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
}
impl VersionKeyInput {
    /// Creates a new builder-style object to manufacture [`VersionKeyInput`](crate::operation::version_key::builders::VersionKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::operation::version_key::builders::VersionKeyInputBuilder{
        crate::deps::aws_cryptography_keyStore::operation::version_key::builders::VersionKeyInputBuilder::default()
    }
}

/// A builder for [`VersionKeyInput`](crate::operation::operation::VersionKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VersionKeyInputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl VersionKeyInputBuilder {
    /// The identifier for the Branch Key to be versioned.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the Branch Key to be versioned.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// The identifier for the Branch Key to be versioned.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Consumes the builder and constructs a [`VersionKeyInput`](crate::operation::operation::VersionKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyInput {
                branch_key_identifier: self.branch_key_identifier,
            },
        )
    }
}
