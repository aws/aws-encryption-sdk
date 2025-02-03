// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for getting a AWS KMS Client.
pub struct GetClientInput {
    /// The region the client should be created in.
    pub region: ::std::option::Option<::std::string::String>,
}
impl GetClientInput {
    /// The region the client should be created in.
    pub fn region(&self) -> &::std::option::Option<::std::string::String> {
        &self.region
    }
}
impl GetClientInput {
    /// Creates a new builder-style object to manufacture [`GetClientInput`](crate::deps::aws_cryptography_materialProviders::types::GetClientInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::GetClientInputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::GetClientInputBuilder::default()
    }
}

/// A builder for [`GetClientInput`](crate::deps::aws_cryptography_materialProviders::types::GetClientInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetClientInputBuilder {
    pub(crate) region: ::std::option::Option<::std::string::String>,
}
impl GetClientInputBuilder {
    /// The region the client should be created in.
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// The region the client should be created in.
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// The region the client should be created in.
    pub fn get_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.region
    }
    /// Consumes the builder and constructs a [`GetClientInput`](crate::deps::aws_cryptography_materialProviders::types::GetClientInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetClientInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetClientInput {
                region: self.region,
            },
        )
    }
}
