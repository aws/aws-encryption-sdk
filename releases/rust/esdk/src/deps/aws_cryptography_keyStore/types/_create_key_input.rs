// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateKeyInput {
    /// The identifier for the created Branch Key.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
    /// Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateKeyInput {
    /// The identifier for the created Branch Key.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub fn encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
}
impl CreateKeyInput {
    /// Creates a new builder-style object to manufacture [`CreateKeyInput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::CreateKeyInputBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::CreateKeyInputBuilder::default()
    }
}

/// A builder for [`CreateKeyInput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyInputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateKeyInputBuilder {
    /// The identifier for the created Branch Key.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the created Branch Key.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// The identifier for the created Branch Key.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    /// Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = input;
        self
    }
    /// Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    /// Consumes the builder and constructs a [`CreateKeyInput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::CreateKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::CreateKeyInput {
                branch_key_identifier: self.branch_key_identifier,
                encryption_context: self.encryption_context,
            },
        )
    }
}
