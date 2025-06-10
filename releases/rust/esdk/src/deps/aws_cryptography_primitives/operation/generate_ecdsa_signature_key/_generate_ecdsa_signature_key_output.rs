// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateEcdsaSignatureKeyOutput {
    #[allow(missing_docs)]
    pub signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    #[allow(missing_docs)]
    pub signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateEcdsaSignatureKeyOutput {
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
    #[allow(missing_docs)]
    pub fn verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.verification_key
    }
}
impl GenerateEcdsaSignatureKeyOutput {
    /// Creates a new builder-style object to manufacture [`GenerateEcdsaSignatureKeyOutput`](crate::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyOutputBuilder::default()
    }
}

/// A builder for [`GenerateEcdsaSignatureKeyOutput`](crate::operation::operation::GenerateEcdsaSignatureKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEcdsaSignatureKeyOutputBuilder {
    pub(crate) signature_algorithm: ::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
    pub(crate) signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateEcdsaSignatureKeyOutputBuilder {
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
    /// Consumes the builder and constructs a [`GenerateEcdsaSignatureKeyOutput`](crate::operation::operation::GenerateEcdsaSignatureKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput {
            signature_algorithm: self.signature_algorithm,
signing_key: self.signing_key,
verification_key: self.verification_key,
        })
    }
}
