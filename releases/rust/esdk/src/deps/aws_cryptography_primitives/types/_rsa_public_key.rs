// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct RsaPublicKey {
    #[allow(missing_docs)]
    pub length_bits: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub pem: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaPublicKey {
    #[allow(missing_docs)]
    pub fn length_bits(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length_bits
    }
    #[allow(missing_docs)]
    pub fn pem(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.pem
    }
}
impl RsaPublicKey {
    /// Creates a new builder-style object to manufacture [`RsaPublicKey`](crate::deps::aws_cryptography_primitives::types::RsaPublicKey).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::RsaPublicKeyBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::RsaPublicKeyBuilder::default()
    }
}

/// A builder for [`RsaPublicKey`](crate::deps::aws_cryptography_primitives::types::RsaPublicKey).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RsaPublicKeyBuilder {
    pub(crate) length_bits: ::std::option::Option<::std::primitive::i32>,
    pub(crate) pem: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaPublicKeyBuilder {
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
    #[allow(missing_docs)]
    pub fn pem(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.pem = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_pem(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.pem = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_pem(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.pem
    }
    /// Consumes the builder and constructs a [`RsaPublicKey`](crate::deps::aws_cryptography_primitives::types::RsaPublicKey).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::RsaPublicKey,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::RsaPublicKey {
                length_bits: self.length_bits,
                pem: self.pem,
            },
        )
    }
}
