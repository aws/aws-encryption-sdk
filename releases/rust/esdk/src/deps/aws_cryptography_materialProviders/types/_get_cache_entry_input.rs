// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetCacheEntryInput {
    #[allow(missing_docs)]
    pub bytes_used: ::std::option::Option<::std::primitive::i64>,
    #[allow(missing_docs)]
    pub identifier: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetCacheEntryInput {
    #[allow(missing_docs)]
    pub fn bytes_used(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.bytes_used
    }
    #[allow(missing_docs)]
    pub fn identifier(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.identifier
    }
}
impl GetCacheEntryInput {
    /// Creates a new builder-style object to manufacture [`GetCacheEntryInput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::GetCacheEntryInputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::GetCacheEntryInputBuilder::default()
    }
}

/// A builder for [`GetCacheEntryInput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCacheEntryInputBuilder {
    pub(crate) bytes_used: ::std::option::Option<::std::primitive::i64>,
    pub(crate) identifier: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetCacheEntryInputBuilder {
    #[allow(missing_docs)]
    pub fn bytes_used(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
        self.bytes_used = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_bytes_used(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
        self.bytes_used = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_bytes_used(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.bytes_used
    }
    #[allow(missing_docs)]
    pub fn identifier(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_identifier(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.identifier = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_identifier(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.identifier
    }
    /// Consumes the builder and constructs a [`GetCacheEntryInput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput {
                bytes_used: self.bytes_used,
                identifier: self.identifier,
            },
        )
    }
}
