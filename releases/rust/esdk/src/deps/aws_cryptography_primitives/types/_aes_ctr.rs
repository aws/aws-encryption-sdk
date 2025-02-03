// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AesCtr {
    #[allow(missing_docs)]
    pub key_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub nonce_length: ::std::option::Option<::std::primitive::i32>,
}
impl AesCtr {
    #[allow(missing_docs)]
    pub fn key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.key_length
    }
    #[allow(missing_docs)]
    pub fn nonce_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.nonce_length
    }
}
impl AesCtr {
    /// Creates a new builder-style object to manufacture [`AesCtr`](crate::deps::aws_cryptography_primitives::types::AesCtr).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::AesCtrBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::AesCtrBuilder::default()
    }
}

/// A builder for [`AesCtr`](crate::deps::aws_cryptography_primitives::types::AesCtr).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AesCtrBuilder {
    pub(crate) key_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) nonce_length: ::std::option::Option<::std::primitive::i32>,
}
impl AesCtrBuilder {
    #[allow(missing_docs)]
    pub fn key_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.key_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.key_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.key_length
    }
    #[allow(missing_docs)]
    pub fn nonce_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.nonce_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_nonce_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.nonce_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_nonce_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.nonce_length
    }
    /// Consumes the builder and constructs a [`AesCtr`](crate::deps::aws_cryptography_primitives::types::AesCtr).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::AesCtr,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::types::AesCtr {
            key_length: self.key_length,
            nonce_length: self.nonce_length,
        })
    }
}
