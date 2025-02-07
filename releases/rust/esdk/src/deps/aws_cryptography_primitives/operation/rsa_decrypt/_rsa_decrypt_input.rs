// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct RsaDecryptInput {
    #[allow(missing_docs)]
    pub cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub padding:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
    #[allow(missing_docs)]
    pub private_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaDecryptInput {
    #[allow(missing_docs)]
    pub fn cipher_text(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_text
    }
    #[allow(missing_docs)]
    pub fn padding(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>
    {
        &self.padding
    }
    #[allow(missing_docs)]
    pub fn private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.private_key
    }
}
impl RsaDecryptInput {
    /// Creates a new builder-style object to manufacture [`RsaDecryptInput`](crate::operation::rsa_decrypt::builders::RsaDecryptInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptInputBuilder::default()
    }
}

/// A builder for [`RsaDecryptInput`](crate::operation::operation::RsaDecryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RsaDecryptInputBuilder {
    pub(crate) cipher_text: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) padding:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
    pub(crate) private_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaDecryptInputBuilder {
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
    #[allow(missing_docs)]
    pub fn padding(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.padding = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_padding(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.padding = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_padding(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>
    {
        &self.padding
    }
    #[allow(missing_docs)]
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.private_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.private_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.private_key
    }
    /// Consumes the builder and constructs a [`RsaDecryptInput`](crate::operation::operation::RsaDecryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput {
                cipher_text: self.cipher_text,
                padding: self.padding,
                private_key: self.private_key,
            },
        )
    }
}
