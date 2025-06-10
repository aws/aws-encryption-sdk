// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateRsaKeyPairInput {
    #[allow(missing_docs)]
    pub length_bits: ::std::option::Option<::std::primitive::i32>,
}
impl GenerateRsaKeyPairInput {
    #[allow(missing_docs)]
    pub fn length_bits(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length_bits
    }
}
impl GenerateRsaKeyPairInput {
    /// Creates a new builder-style object to manufacture [`GenerateRsaKeyPairInput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::GenerateRsaKeyPairInputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::GenerateRsaKeyPairInputBuilder::default()
    }
}

/// A builder for [`GenerateRsaKeyPairInput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRsaKeyPairInputBuilder {
    pub(crate) length_bits: ::std::option::Option<::std::primitive::i32>,
}
impl GenerateRsaKeyPairInputBuilder {
    #[allow(missing_docs)]
    pub fn length_bits(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.length_bits = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_length_bits(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.length_bits = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_length_bits(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length_bits
    }
    /// Consumes the builder and constructs a [`GenerateRsaKeyPairInput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput {
                length_bits: self.length_bits,
            },
        )
    }
}
