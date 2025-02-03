// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::_rsa_decrypt_output::RsaDecryptOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::_rsa_decrypt_input::RsaDecryptInputBuilder;

impl RsaDecryptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let mut fluent_builder = client.rsa_decrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RsaDecrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RsaDecryptFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptInputBuilder,
}
impl RsaDecryptFluentBuilder {
    /// Creates a new `RsaDecrypt`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the RsaDecrypt as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
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
        crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecrypt::send(
            &self.client,
            input,
        )
        .await
    }

    #[allow(missing_docs)]
    pub fn cipher_text(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.cipher_text(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_cipher_text(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_cipher_text(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_cipher_text(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_cipher_text()
    }
    #[allow(missing_docs)]
    pub fn padding(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.inner = self.inner.padding(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_padding(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.inner = self.inner.set_padding(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_padding(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>
    {
        self.inner.get_padding()
    }
    #[allow(missing_docs)]
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.private_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_private_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_private_key()
    }
}
