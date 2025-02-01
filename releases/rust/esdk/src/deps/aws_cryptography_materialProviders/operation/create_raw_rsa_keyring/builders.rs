// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::_create_raw_rsa_keyring_input::CreateRawRsaKeyringInputBuilder;

impl CreateRawRsaKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_raw_rsa_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRawRsaKeyring`.
///
/// Creates a Raw RSA Keyring, which wraps and unwraps data keys locally using RSA.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRawRsaKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder,
}
impl CreateRawRsaKeyringFluentBuilder {
    /// Creates a new `CreateRawRsaKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateRawRsaKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyring::send(&self.client, input).await
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
    /// The RSA padding scheme to use with this keyring.
    pub fn padding_scheme(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
        >,
    ) -> Self {
        self.inner = self.inner.padding_scheme(input.into());
        self
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn set_padding_scheme(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
        >,
    ) -> Self {
        self.inner = self.inner.set_padding_scheme(input);
        self
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn get_padding_scheme(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::PaddingScheme>
    {
        self.inner.get_padding_scheme()
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.private_key(input.into());
        self
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_private_key(input);
        self
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn get_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_private_key()
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.public_key(input.into());
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_public_key(input);
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_public_key()
    }
}
