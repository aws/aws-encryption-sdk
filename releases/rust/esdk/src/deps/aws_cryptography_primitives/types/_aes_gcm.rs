// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AesGcm {
    #[allow(missing_docs)]
    pub iv_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub key_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub tag_length: ::std::option::Option<::std::primitive::i32>,
}
impl AesGcm {
    #[allow(missing_docs)]
    pub fn iv_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.iv_length
    }
    #[allow(missing_docs)]
    pub fn key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.key_length
    }
    #[allow(missing_docs)]
    pub fn tag_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.tag_length
    }
}
impl AesGcm {
    /// Creates a new builder-style object to manufacture [`AesGcm`](crate::deps::aws_cryptography_primitives::types::AesGcm).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::AesGcmBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::AesGcmBuilder::default()
    }
}

/// A builder for [`AesGcm`](crate::deps::aws_cryptography_primitives::types::AesGcm).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AesGcmBuilder {
    pub(crate) iv_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) key_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) tag_length: ::std::option::Option<::std::primitive::i32>,
}
impl AesGcmBuilder {
    #[allow(missing_docs)]
    pub fn iv_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.iv_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_iv_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.iv_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_iv_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.iv_length
    }
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
    pub fn tag_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.tag_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_tag_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.tag_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_tag_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.tag_length
    }
    /// Consumes the builder and constructs a [`AesGcm`](crate::deps::aws_cryptography_primitives::types::AesGcm).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::AesGcm,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::types::AesGcm {
            iv_length: self.iv_length,
            key_length: self.key_length,
            tag_length: self.tag_length,
        })
    }
}
