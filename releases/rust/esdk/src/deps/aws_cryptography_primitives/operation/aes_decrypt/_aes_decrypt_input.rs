// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AesDecryptInput {
    #[allow(missing_docs)]
    pub aad: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub auth_tag: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub cipher_txt: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub enc_alg: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    #[allow(missing_docs)]
    pub iv: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesDecryptInput {
    #[allow(missing_docs)]
    pub fn aad(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.aad
    }
    #[allow(missing_docs)]
    pub fn auth_tag(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.auth_tag
    }
    #[allow(missing_docs)]
    pub fn cipher_txt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_txt
    }
    #[allow(missing_docs)]
    pub fn enc_alg(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm> {
        &self.enc_alg
    }
    #[allow(missing_docs)]
    pub fn iv(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.iv
    }
    #[allow(missing_docs)]
    pub fn key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key
    }
}
impl AesDecryptInput {
    /// Creates a new builder-style object to manufacture [`AesDecryptInput`](crate::operation::aes_decrypt::builders::AesDecryptInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptInputBuilder::default()
    }
}

/// A builder for [`AesDecryptInput`](crate::operation::operation::AesDecryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AesDecryptInputBuilder {
    pub(crate) aad: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) auth_tag: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) cipher_txt: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) enc_alg:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    pub(crate) iv: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl AesDecryptInputBuilder {
    #[allow(missing_docs)]
    pub fn aad(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.aad = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_aad(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.aad = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_aad(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.aad
    }
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
    pub fn cipher_txt(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.cipher_txt = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_cipher_txt(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.cipher_txt = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_cipher_txt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.cipher_txt
    }
    #[allow(missing_docs)]
    pub fn enc_alg(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    ) -> Self {
        self.enc_alg = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_enc_alg(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    ) -> Self {
        self.enc_alg = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_enc_alg(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm> {
        &self.enc_alg
    }
    #[allow(missing_docs)]
    pub fn iv(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.iv = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_iv(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.iv = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_iv(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.iv
    }
    #[allow(missing_docs)]
    pub fn key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key
    }
    /// Consumes the builder and constructs a [`AesDecryptInput`](crate::operation::operation::AesDecryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptInput {
                aad: self.aad,
                auth_tag: self.auth_tag,
                cipher_txt: self.cipher_txt,
                enc_alg: self.enc_alg,
                iv: self.iv,
                key: self.key,
            },
        )
    }
}
