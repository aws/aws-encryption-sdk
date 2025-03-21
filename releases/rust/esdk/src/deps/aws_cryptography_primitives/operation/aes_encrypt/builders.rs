// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::aes_encrypt::_aes_encrypt_output::AesEncryptOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::aes_encrypt::_aes_encrypt_input::AesEncryptInputBuilder;

impl AesEncryptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let mut fluent_builder = client.aes_encrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AesEncrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AesEncryptFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptInputBuilder,
}
impl AesEncryptFluentBuilder {
    /// Creates a new `AesEncrypt`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the AesEncrypt as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
                let msg = format!("{:?}", e);
                crate::deps::aws_cryptography_primitives::types::error::Error::OpaqueWithText {
                    obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
                    objMessage: msg,
                }
            })?;
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncrypt::send(
            &self.client,
            input,
        )
        .await
    }

    #[allow(missing_docs)]
    pub fn aad(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.aad(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_aad(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_aad(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_aad(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_aad()
    }
    #[allow(missing_docs)]
    pub fn enc_alg(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    ) -> Self {
        self.inner = self.inner.enc_alg(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_enc_alg(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
    ) -> Self {
        self.inner = self.inner.set_enc_alg(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_enc_alg(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm> {
        self.inner.get_enc_alg()
    }
    #[allow(missing_docs)]
    pub fn iv(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.iv(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_iv(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_iv(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_iv(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_iv()
    }
    #[allow(missing_docs)]
    pub fn key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_key()
    }
    #[allow(missing_docs)]
    pub fn msg(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.msg(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_msg(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_msg(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_msg(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_msg()
    }
}
