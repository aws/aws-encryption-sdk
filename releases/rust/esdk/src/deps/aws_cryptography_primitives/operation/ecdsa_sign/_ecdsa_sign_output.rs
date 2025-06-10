// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EcdsaSignOutput {
    #[allow(missing_docs)]
    pub signature: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaSignOutput {
    #[allow(missing_docs)]
    pub fn signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signature
    }
}
impl EcdsaSignOutput {
    /// Creates a new builder-style object to manufacture [`EcdsaSignOutput`](crate::operation::ecdsa_sign::builders::EcdsaSignOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignOutputBuilder::default()
    }
}

/// A builder for [`EcdsaSignOutput`](crate::operation::operation::EcdsaSignOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EcdsaSignOutputBuilder {
    pub(crate) signature: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EcdsaSignOutputBuilder {
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
    /// Consumes the builder and constructs a [`EcdsaSignOutput`](crate::operation::operation::EcdsaSignOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignOutput {
                signature: self.signature,
            },
        )
    }
}
