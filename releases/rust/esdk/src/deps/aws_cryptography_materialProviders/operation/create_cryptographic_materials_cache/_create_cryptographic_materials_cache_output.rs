// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateCryptographicMaterialsCacheOutput {
    #[allow(missing_docs)]
pub materials_cache: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>,
}
impl CreateCryptographicMaterialsCacheOutput {
    #[allow(missing_docs)]
    pub fn materials_cache(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>{
        &self.materials_cache
    }
}
impl CreateCryptographicMaterialsCacheOutput {
    /// Creates a new builder-style object to manufacture [`CreateCryptographicMaterialsCacheOutput`](crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheOutputBuilder::default()
    }
}

/// A builder for [`CreateCryptographicMaterialsCacheOutput`](crate::operation::operation::CreateCryptographicMaterialsCacheOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCryptographicMaterialsCacheOutputBuilder {
    pub(crate) materials_cache: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>,
}
impl CreateCryptographicMaterialsCacheOutputBuilder {
    #[allow(missing_docs)]
    pub fn materials_cache(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>,
    ) -> Self {
        self.materials_cache = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials_cache(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>,
    ) -> Self {
        self.materials_cache = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials_cache(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>{
        &self.materials_cache
    }
    /// Consumes the builder and constructs a [`CreateCryptographicMaterialsCacheOutput`](crate::operation::operation::CreateCryptographicMaterialsCacheOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput {
            materials_cache: self.materials_cache,
        })
    }
}
