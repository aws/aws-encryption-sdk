// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateEcdsaSignatureKeyInput {
    #[allow(missing_docs)]
    pub signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
}
impl GenerateEcdsaSignatureKeyInput {
    #[allow(missing_docs)]
    pub fn signature_algorithm(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        &self.signature_algorithm
    }
}
impl GenerateEcdsaSignatureKeyInput {
    /// Creates a new builder-style object to manufacture [`GenerateEcdsaSignatureKeyInput`](crate::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder::default()
    }
}

/// A builder for [`GenerateEcdsaSignatureKeyInput`](crate::operation::operation::GenerateEcdsaSignatureKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEcdsaSignatureKeyInputBuilder {
    pub(crate) signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
}
impl GenerateEcdsaSignatureKeyInputBuilder {
    #[allow(missing_docs)]
    pub fn signature_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.signature_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.signature_algorithm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature_algorithm(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        &self.signature_algorithm
    }
    /// Consumes the builder and constructs a [`GenerateEcdsaSignatureKeyInput`](crate::operation::operation::GenerateEcdsaSignatureKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput {
            signature_algorithm: self.signature_algorithm,
        })
    }
}
