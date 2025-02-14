// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CompressPublicKeyOutput {
    #[allow(missing_docs)]
    pub compressed_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CompressPublicKeyOutput {
    #[allow(missing_docs)]
    pub fn compressed_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.compressed_public_key
    }
}
impl CompressPublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`CompressPublicKeyOutput`](crate::operation::compress_public_key::builders::CompressPublicKeyOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyOutputBuilder::default()
    }
}

/// A builder for [`CompressPublicKeyOutput`](crate::operation::operation::CompressPublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CompressPublicKeyOutputBuilder {
    pub(crate) compressed_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CompressPublicKeyOutputBuilder {
    #[allow(missing_docs)]
    pub fn compressed_public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.compressed_public_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_compressed_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.compressed_public_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_compressed_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.compressed_public_key
    }
    /// Consumes the builder and constructs a [`CompressPublicKeyOutput`](crate::operation::operation::CompressPublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyOutput {
            compressed_public_key: self.compressed_public_key,
        })
    }
}
