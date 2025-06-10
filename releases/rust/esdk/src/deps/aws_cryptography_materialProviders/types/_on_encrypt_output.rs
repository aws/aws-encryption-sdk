// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OnEncryptOutput {
    #[allow(missing_docs)]
    pub materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl OnEncryptOutput {
    #[allow(missing_docs)]
    pub fn materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.materials
    }
}
impl OnEncryptOutput {
    /// Creates a new builder-style object to manufacture [`OnEncryptOutput`](crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::OnEncryptOutputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::OnEncryptOutputBuilder::default()
    }
}

/// A builder for [`OnEncryptOutput`](crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OnEncryptOutputBuilder {
    pub(crate) materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl OnEncryptOutputBuilder {
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
    /// Consumes the builder and constructs a [`OnEncryptOutput`](crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput {
                materials: self.materials,
            },
        )
    }
}
