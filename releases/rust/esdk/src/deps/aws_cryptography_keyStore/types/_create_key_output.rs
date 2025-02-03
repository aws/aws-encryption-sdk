// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for Branch Key creation.
pub struct CreateKeyOutput {
    /// A identifier for the created Branch Key.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl CreateKeyOutput {
    /// A identifier for the created Branch Key.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
}
impl CreateKeyOutput {
    /// Creates a new builder-style object to manufacture [`CreateKeyOutput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::CreateKeyOutputBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::CreateKeyOutputBuilder::default()
    }
}

/// A builder for [`CreateKeyOutput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyOutputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl CreateKeyOutputBuilder {
    /// A identifier for the created Branch Key.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// A identifier for the created Branch Key.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// A identifier for the created Branch Key.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Consumes the builder and constructs a [`CreateKeyOutput`](crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput {
                branch_key_identifier: self.branch_key_identifier,
            },
        )
    }
}
