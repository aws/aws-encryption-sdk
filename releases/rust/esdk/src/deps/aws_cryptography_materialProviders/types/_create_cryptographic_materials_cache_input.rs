// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateCryptographicMaterialsCacheInput {
    /// Which type of local cache to use.
    pub cache:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>,
}
impl CreateCryptographicMaterialsCacheInput {
    /// Which type of local cache to use.
    pub fn cache(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>
    {
        &self.cache
    }
}
impl CreateCryptographicMaterialsCacheInput {
    /// Creates a new builder-style object to manufacture [`CreateCryptographicMaterialsCacheInput`](crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::CreateCryptographicMaterialsCacheInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::CreateCryptographicMaterialsCacheInputBuilder::default()
    }
}

/// A builder for [`CreateCryptographicMaterialsCacheInput`](crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCryptographicMaterialsCacheInputBuilder {
    pub(crate) cache:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>,
}
impl CreateCryptographicMaterialsCacheInputBuilder {
    /// Which type of local cache to use.
    pub fn cache(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.cache = ::std::option::Option::Some(input.into());
        self
    }
    /// Which type of local cache to use.
    pub fn set_cache(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.cache = input;
        self
    }
    /// Which type of local cache to use.
    pub fn get_cache(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>
    {
        &self.cache
    }
    /// Consumes the builder and constructs a [`CreateCryptographicMaterialsCacheInput`](crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput {
            cache: self.cache,
        })
    }
}
