// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecryptMaterialsOutput {
    #[allow(missing_docs)]
    pub decryption_materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl DecryptMaterialsOutput {
    #[allow(missing_docs)]
    pub fn decryption_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.decryption_materials
    }
}
impl DecryptMaterialsOutput {
    /// Creates a new builder-style object to manufacture [`DecryptMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::DecryptMaterialsOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::DecryptMaterialsOutputBuilder::default()
    }
}

/// A builder for [`DecryptMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptMaterialsOutputBuilder {
    pub(crate) decryption_materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl DecryptMaterialsOutputBuilder {
    #[allow(missing_docs)]
    pub fn decryption_materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.decryption_materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_decryption_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.decryption_materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_decryption_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.decryption_materials
    }
    /// Consumes the builder and constructs a [`DecryptMaterialsOutput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput {
                decryption_materials: self.decryption_materials,
            },
        )
    }
}
