// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct ValidEncryptionMaterialsTransitionInput {
    #[allow(missing_docs)]
    pub start: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
    #[allow(missing_docs)]
    pub stop: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl ValidEncryptionMaterialsTransitionInput {
    #[allow(missing_docs)]
    pub fn start(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.start
    }
    #[allow(missing_docs)]
    pub fn stop(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.stop
    }
}
impl ValidEncryptionMaterialsTransitionInput {
    /// Creates a new builder-style object to manufacture [`ValidEncryptionMaterialsTransitionInput`](crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::ValidEncryptionMaterialsTransitionInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::ValidEncryptionMaterialsTransitionInputBuilder::default()
    }
}

/// A builder for [`ValidEncryptionMaterialsTransitionInput`](crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidEncryptionMaterialsTransitionInputBuilder {
    pub(crate) start: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
    pub(crate) stop: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
}
impl ValidEncryptionMaterialsTransitionInputBuilder {
    #[allow(missing_docs)]
    pub fn start(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.start = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_start(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.start = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_start(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.start
    }
    #[allow(missing_docs)]
    pub fn stop(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.stop = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_stop(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.stop = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_stop(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        &self.stop
    }
    /// Consumes the builder and constructs a [`ValidEncryptionMaterialsTransitionInput`](crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput {
            start: self.start,
stop: self.stop,
        })
    }
}
