// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetEncryptionMaterialsOutput {
    #[allow(missing_docs)]
    pub encryption_materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl GetEncryptionMaterialsOutput {
    #[allow(missing_docs)]
    pub fn encryption_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.encryption_materials
    }
}
impl GetEncryptionMaterialsOutput {
    /// Creates a new builder-style object to manufacture [`GetEncryptionMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::GetEncryptionMaterialsOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::GetEncryptionMaterialsOutputBuilder::default()
    }
}

/// A builder for [`GetEncryptionMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetEncryptionMaterialsOutputBuilder {
    pub(crate) encryption_materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl GetEncryptionMaterialsOutputBuilder {
    #[allow(missing_docs)]
    pub fn encryption_materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.encryption_materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encryption_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.encryption_materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encryption_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.encryption_materials
    }
    /// Consumes the builder and constructs a [`GetEncryptionMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput {
                encryption_materials: self.encryption_materials,
            },
        )
    }
}
