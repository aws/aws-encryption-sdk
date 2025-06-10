// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for determining the Branch Key which should be used to wrap or unwrap the data key for this encryption or decryption
pub struct GetBranchKeyIdInput {
    /// The Encryption Context used with this encryption or decryption.
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl GetBranchKeyIdInput {
    /// The Encryption Context used with this encryption or decryption.
    pub fn encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
}
impl GetBranchKeyIdInput {
    /// Creates a new builder-style object to manufacture [`GetBranchKeyIdInput`](crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::GetBranchKeyIdInputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::GetBranchKeyIdInputBuilder::default()
    }
}

/// A builder for [`GetBranchKeyIdInput`](crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBranchKeyIdInputBuilder {
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl GetBranchKeyIdInputBuilder {
    /// The Encryption Context used with this encryption or decryption.
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    /// The Encryption Context used with this encryption or decryption.
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = input;
        self
    }
    /// The Encryption Context used with this encryption or decryption.
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    /// Consumes the builder and constructs a [`GetBranchKeyIdInput`](crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput {
                encryption_context: self.encryption_context,
            },
        )
    }
}
