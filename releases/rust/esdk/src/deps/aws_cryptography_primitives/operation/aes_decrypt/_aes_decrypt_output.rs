// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AesDecryptOutput {
    #[allow(missing_docs)]
    pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesDecryptOutput {
    #[allow(missing_docs)]
    pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
}
impl AesDecryptOutput {
    /// Creates a new builder-style object to manufacture [`AesDecryptOutput`](crate::operation::aes_decrypt::builders::AesDecryptOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptOutputBuilder::default()
    }
}

/// A builder for [`AesDecryptOutput`](crate::operation::operation::AesDecryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AesDecryptOutputBuilder {
    pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesDecryptOutputBuilder {
    #[allow(missing_docs)]
    pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
    /// Consumes the builder and constructs a [`AesDecryptOutput`](crate::operation::operation::AesDecryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptOutput {
                plaintext: self.plaintext,
            },
        )
    }
}
