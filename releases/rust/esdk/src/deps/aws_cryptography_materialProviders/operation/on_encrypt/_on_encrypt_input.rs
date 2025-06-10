// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OnEncryptInput {
    #[allow(missing_docs)]
    pub materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl OnEncryptInput {
    #[allow(missing_docs)]
    pub fn materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.materials
    }
}
impl OnEncryptInput {
    /// Creates a new builder-style object to manufacture [`OnEncryptInput`](crate::operation::on_encrypt::builders::OnEncryptInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::builders::OnEncryptInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::builders::OnEncryptInputBuilder::default()
    }
}

/// A builder for [`OnEncryptInput`](crate::operation::operation::OnEncryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OnEncryptInputBuilder {
    pub(crate) materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl OnEncryptInputBuilder {
    #[allow(missing_docs)]
    pub fn materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.materials
    }
    /// Consumes the builder and constructs a [`OnEncryptInput`](crate::operation::operation::OnEncryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput {
            materials: self.materials,
        })
    }
}
