// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OnDecryptOutput {
    #[allow(missing_docs)]
    pub materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl OnDecryptOutput {
    #[allow(missing_docs)]
    pub fn materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.materials
    }
}
impl OnDecryptOutput {
    /// Creates a new builder-style object to manufacture [`OnDecryptOutput`](crate::operation::on_decrypt::builders::OnDecryptOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::builders::OnDecryptOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::builders::OnDecryptOutputBuilder::default()
    }
}

/// A builder for [`OnDecryptOutput`](crate::operation::operation::OnDecryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OnDecryptOutputBuilder {
    pub(crate) materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl OnDecryptOutputBuilder {
    #[allow(missing_docs)]
    pub fn materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.materials
    }
    /// Consumes the builder and constructs a [`OnDecryptOutput`](crate::operation::operation::OnDecryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput {
            materials: self.materials,
        })
    }
}
