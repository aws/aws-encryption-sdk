// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DeleteCacheEntryInput {
    #[allow(missing_docs)]
    pub identifier: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DeleteCacheEntryInput {
    #[allow(missing_docs)]
    pub fn identifier(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.identifier
    }
}
impl DeleteCacheEntryInput {
    /// Creates a new builder-style object to manufacture [`DeleteCacheEntryInput`](crate::operation::delete_cache_entry::builders::DeleteCacheEntryInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::builders::DeleteCacheEntryInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::builders::DeleteCacheEntryInputBuilder::default()
    }
}

/// A builder for [`DeleteCacheEntryInput`](crate::operation::operation::DeleteCacheEntryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCacheEntryInputBuilder {
    pub(crate) identifier: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DeleteCacheEntryInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteCacheEntryInput`](crate::operation::operation::DeleteCacheEntryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput {
            identifier: self.identifier,
        })
    }
}
