// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for creating a Default Cryptographic Materials Manager.
pub struct CreateCryptographicMaterialsManagerOutput {
    /// The created Default Cryptographic Materials Manager.
pub materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateCryptographicMaterialsManagerOutput {
    /// The created Default Cryptographic Materials Manager.
    pub fn materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.materials_manager
    }
}
impl CreateCryptographicMaterialsManagerOutput {
    /// Creates a new builder-style object to manufacture [`CreateCryptographicMaterialsManagerOutput`](crate::operation::create_default_cryptographic_materials_manager::builders::CreateCryptographicMaterialsManagerOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateCryptographicMaterialsManagerOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateCryptographicMaterialsManagerOutputBuilder::default()
    }
}

/// A builder for [`CreateCryptographicMaterialsManagerOutput`](crate::operation::operation::CreateCryptographicMaterialsManagerOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCryptographicMaterialsManagerOutputBuilder {
    pub(crate) materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateCryptographicMaterialsManagerOutputBuilder {
    /// The created Default Cryptographic Materials Manager.
    pub fn materials_manager(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.materials_manager = ::std::option::Option::Some(input.into());
        self
    }
    /// The created Default Cryptographic Materials Manager.
    pub fn set_materials_manager(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.materials_manager = input;
        self
    }
    /// The created Default Cryptographic Materials Manager.
    pub fn get_materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.materials_manager
    }
    /// Consumes the builder and constructs a [`CreateCryptographicMaterialsManagerOutput`](crate::operation::operation::CreateCryptographicMaterialsManagerOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput {
            materials_manager: self.materials_manager,
        })
    }
}
