// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct Ecdsa {
    #[allow(missing_docs)]
    pub curve: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
}
impl Ecdsa {
    #[allow(missing_docs)]
    pub fn curve(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        &self.curve
    }
}
impl Ecdsa {
    /// Creates a new builder-style object to manufacture [`Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::Ecdsa).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::EcdsaBuilder {
        crate::deps::aws_cryptography_materialProviders::types::builders::EcdsaBuilder::default()
    }
}

/// A builder for [`Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::Ecdsa).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EcdsaBuilder {
    pub(crate) curve: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
}
impl EcdsaBuilder {
    #[allow(missing_docs)]
    pub fn curve(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.curve = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_curve(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.curve = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_curve(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        &self.curve
    }
    /// Consumes the builder and constructs a [`Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::Ecdsa).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::Ecdsa,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::Ecdsa { curve: self.curve },
        )
    }
}
