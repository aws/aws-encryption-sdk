// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::_create_raw_aes_keyring_input::CreateRawAesKeyringInputBuilder;

impl CreateRawAesKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_raw_aes_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRawAesKeyring`.
///
/// Creates a Raw AES Keyring, which wraps and unwraps data keys locally using AES_GCM.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRawAesKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringInputBuilder,
}
impl CreateRawAesKeyringFluentBuilder {
    /// Creates a new `CreateRawAesKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateRawAesKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
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
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::CreateRawAesKeyring::send(&self.client, input).await
    }

    /// A name associated with this wrapping key.
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// A name associated with this wrapping key.
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// A name associated with this wrapping key.
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_name()
    }
    /// A namespace associated with this wrapping key.
    pub fn key_namespace(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.key_namespace(input.into());
        self
    }
    /// A namespace associated with this wrapping key.
    pub fn set_key_namespace(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_key_namespace(input);
        self
    }
    /// A namespace associated with this wrapping key.
    pub fn get_key_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_namespace()
    }
    /// The AES_GCM algorithm this Keyring uses to wrap and unwrap data keys.
    pub fn wrapping_alg(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg,
        >,
    ) -> Self {
        self.inner = self.inner.wrapping_alg(input.into());
        self
    }
    /// The AES_GCM algorithm this Keyring uses to wrap and unwrap data keys.
    pub fn set_wrapping_alg(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg,
        >,
    ) -> Self {
        self.inner = self.inner.set_wrapping_alg(input);
        self
    }
    /// The AES_GCM algorithm this Keyring uses to wrap and unwrap data keys.
    pub fn get_wrapping_alg(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg,
    > {
        self.inner.get_wrapping_alg()
    }
    /// The AES key used with AES_GCM encryption and decryption.
    pub fn wrapping_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.wrapping_key(input.into());
        self
    }
    /// The AES key used with AES_GCM encryption and decryption.
    pub fn set_wrapping_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_wrapping_key(input);
        self
    }
    /// The AES key used with AES_GCM encryption and decryption.
    pub fn get_wrapping_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_wrapping_key()
    }
}
