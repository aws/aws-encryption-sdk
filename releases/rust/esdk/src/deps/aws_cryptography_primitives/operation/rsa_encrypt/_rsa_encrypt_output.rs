// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct RsaEncryptOutput {
    #[allow(missing_docs)]
    pub cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaEncryptOutput {
    #[allow(missing_docs)]
    pub fn cipher_text(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_text
    }
}
impl RsaEncryptOutput {
    /// Creates a new builder-style object to manufacture [`RsaEncryptOutput`](crate::operation::rsa_encrypt::builders::RsaEncryptOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::builders::RsaEncryptOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::builders::RsaEncryptOutputBuilder::default()
    }
}

/// A builder for [`RsaEncryptOutput`](crate::operation::operation::RsaEncryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RsaEncryptOutputBuilder {
    pub(crate) cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaEncryptOutputBuilder {
    #[allow(missing_docs)]
    pub fn cipher_text(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.cipher_text = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_cipher_text(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.cipher_text = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_cipher_text(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_text
    }
    /// Consumes the builder and constructs a [`RsaEncryptOutput`](crate::operation::operation::RsaEncryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptOutput {
                cipher_text: self.cipher_text,
            },
        )
    }
}
