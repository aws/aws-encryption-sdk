// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct ValidDecryptionMaterialsTransitionInput {
    #[allow(missing_docs)]
    pub start: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
    #[allow(missing_docs)]
    pub stop: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl ValidDecryptionMaterialsTransitionInput {
    #[allow(missing_docs)]
    pub fn start(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.start
    }
    #[allow(missing_docs)]
    pub fn stop(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.stop
    }
}
impl ValidDecryptionMaterialsTransitionInput {
    /// Creates a new builder-style object to manufacture [`ValidDecryptionMaterialsTransitionInput`](crate::operation::valid_decryption_materials_transition::builders::ValidDecryptionMaterialsTransitionInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::builders::ValidDecryptionMaterialsTransitionInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::builders::ValidDecryptionMaterialsTransitionInputBuilder::default()
    }
}

/// A builder for [`ValidDecryptionMaterialsTransitionInput`](crate::operation::operation::ValidDecryptionMaterialsTransitionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidDecryptionMaterialsTransitionInputBuilder {
    pub(crate) start: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
    pub(crate) stop: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl ValidDecryptionMaterialsTransitionInputBuilder {
    #[allow(missing_docs)]
    pub fn start(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.start = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_start(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.start = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_start(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.start
    }
    #[allow(missing_docs)]
    pub fn stop(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.stop = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_stop(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.stop = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_stop(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.stop
    }
    /// Consumes the builder and constructs a [`ValidDecryptionMaterialsTransitionInput`](crate::operation::operation::ValidDecryptionMaterialsTransitionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput {
            start: self.start,
stop: self.stop,
        })
    }
}
