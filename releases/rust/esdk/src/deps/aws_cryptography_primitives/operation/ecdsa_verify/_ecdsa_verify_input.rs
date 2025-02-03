// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EcdsaVerifyInput {
    #[allow(missing_docs)]
    pub message: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub signature: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    #[allow(missing_docs)]
    pub verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaVerifyInput {
    #[allow(missing_docs)]
    pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
    #[allow(missing_docs)]
    pub fn signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signature
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
    pub fn verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.verification_key
    }
}
impl EcdsaVerifyInput {
    /// Creates a new builder-style object to manufacture [`EcdsaVerifyInput`](crate::operation::ecdsa_verify::builders::EcdsaVerifyInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyInputBuilder::default()
    }
}

/// A builder for [`EcdsaVerifyInput`](crate::operation::operation::EcdsaVerifyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EcdsaVerifyInputBuilder {
    pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) signature: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    pub(crate) verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaVerifyInputBuilder {
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
    pub fn signature(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.signature = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.signature = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signature
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
    pub fn verification_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.verification_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_verification_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.verification_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.verification_key
    }
    /// Consumes the builder and constructs a [`EcdsaVerifyInput`](crate::operation::operation::EcdsaVerifyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput {
                message: self.message,
                signature: self.signature,
                signature_algorithm: self.signature_algorithm,
                verification_key: self.verification_key,
            },
        )
    }
}
