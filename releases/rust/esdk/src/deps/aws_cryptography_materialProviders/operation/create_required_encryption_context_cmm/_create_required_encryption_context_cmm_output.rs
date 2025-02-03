// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for creating an Required Encryption Context Cryptographic Materials Manager.
pub struct CreateRequiredEncryptionContextCmmOutput {
    /// The created Required Encryption Context Cryptographic Materials Manager.
pub materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateRequiredEncryptionContextCmmOutput {
    /// The created Required Encryption Context Cryptographic Materials Manager.
    pub fn materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.materials_manager
    }
}
impl CreateRequiredEncryptionContextCmmOutput {
    /// Creates a new builder-style object to manufacture [`CreateRequiredEncryptionContextCmmOutput`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmOutputBuilder::default()
    }
}

/// A builder for [`CreateRequiredEncryptionContextCmmOutput`](crate::operation::operation::CreateRequiredEncryptionContextCmmOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRequiredEncryptionContextCmmOutputBuilder {
    pub(crate) materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateRequiredEncryptionContextCmmOutputBuilder {
    /// The created Required Encryption Context Cryptographic Materials Manager.
    pub fn materials_manager(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.materials_manager = ::std::option::Option::Some(input.into());
        self
    }
    /// The created Required Encryption Context Cryptographic Materials Manager.
    pub fn set_materials_manager(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.materials_manager = input;
        self
    }
    /// The created Required Encryption Context Cryptographic Materials Manager.
    pub fn get_materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.materials_manager
    }
    /// Consumes the builder and constructs a [`CreateRequiredEncryptionContextCmmOutput`](crate::operation::operation::CreateRequiredEncryptionContextCmmOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmOutput {
            materials_manager: self.materials_manager,
        })
    }
}
