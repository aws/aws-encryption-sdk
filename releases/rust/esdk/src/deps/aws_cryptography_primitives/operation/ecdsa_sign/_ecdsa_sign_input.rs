// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EcdsaSignInput {
    #[allow(missing_docs)]
    pub message: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    #[allow(missing_docs)]
    pub signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaSignInput {
    #[allow(missing_docs)]
    pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
    #[allow(missing_docs)]
    pub fn signature_algorithm(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        &self.signature_algorithm
    }
    #[allow(missing_docs)]
    pub fn signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signing_key
    }
}
impl EcdsaSignInput {
    /// Creates a new builder-style object to manufacture [`EcdsaSignInput`](crate::operation::ecdsa_sign::builders::EcdsaSignInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignInputBuilder::default()
    }
}

/// A builder for [`EcdsaSignInput`](crate::operation::operation::EcdsaSignInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EcdsaSignInputBuilder {
    pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    pub(crate) signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaSignInputBuilder {
    #[allow(missing_docs)]
    pub fn message(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
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
    #[allow(missing_docs)]
    pub fn signing_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.signing_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signing_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.signing_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signing_key
    }
    /// Consumes the builder and constructs a [`EcdsaSignInput`](crate::operation::operation::EcdsaSignInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput {
                message: self.message,
                signature_algorithm: self.signature_algorithm,
                signing_key: self.signing_key,
            },
        )
    }
}
