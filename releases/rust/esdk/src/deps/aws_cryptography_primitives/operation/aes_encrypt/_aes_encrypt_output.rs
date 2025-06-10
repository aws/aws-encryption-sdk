// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AesEncryptOutput {
    #[allow(missing_docs)]
    pub auth_tag: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesEncryptOutput {
    #[allow(missing_docs)]
    pub fn auth_tag(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.auth_tag
    }
    #[allow(missing_docs)]
    pub fn cipher_text(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_text
    }
}
impl AesEncryptOutput {
    /// Creates a new builder-style object to manufacture [`AesEncryptOutput`](crate::operation::aes_encrypt::builders::AesEncryptOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptOutputBuilder::default()
    }
}

/// A builder for [`AesEncryptOutput`](crate::operation::operation::AesEncryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AesEncryptOutputBuilder {
    pub(crate) auth_tag: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesEncryptOutputBuilder {
    #[allow(missing_docs)]
    pub fn auth_tag(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.auth_tag = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_auth_tag(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.auth_tag = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_auth_tag(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.auth_tag
    }
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
    /// Consumes the builder and constructs a [`AesEncryptOutput`](crate::operation::operation::AesEncryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput {
                auth_tag: self.auth_tag,
                cipher_text: self.cipher_text,
            },
        )
    }
}
