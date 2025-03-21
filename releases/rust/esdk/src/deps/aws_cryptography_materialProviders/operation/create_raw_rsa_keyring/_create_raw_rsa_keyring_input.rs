// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a Raw RAW Keyring.
pub struct CreateRawRsaKeyringInput {
    /// A name associated with this wrapping key.
    pub key_name: ::std::option::Option<::std::string::String>,
    /// A namespace associated with this wrapping key.
    pub key_namespace: ::std::option::Option<::std::string::String>,
    /// The RSA padding scheme to use with this keyring.
    pub padding_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
    >,
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub private_key: ::std::option::Option<::aws_smithy_types::Blob>,
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CreateRawRsaKeyringInput {
    /// A name associated with this wrapping key.
    pub fn key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_name
    }
    /// A namespace associated with this wrapping key.
    pub fn key_namespace(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_namespace
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn padding_scheme(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::PaddingScheme>
    {
        &self.padding_scheme
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.private_key
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
}
impl CreateRawRsaKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateRawRsaKeyringInput`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateRawRsaKeyringInput`](crate::operation::operation::CreateRawRsaKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRawRsaKeyringInputBuilder {
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) key_namespace: ::std::option::Option<::std::string::String>,
    pub(crate) padding_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
    >,
    pub(crate) private_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CreateRawRsaKeyringInputBuilder {
    /// A name associated with this wrapping key.
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// A name associated with this wrapping key.
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// A name associated with this wrapping key.
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_name
    }
    /// A namespace associated with this wrapping key.
    pub fn key_namespace(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.key_namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// A namespace associated with this wrapping key.
    pub fn set_key_namespace(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.key_namespace = input;
        self
    }
    /// A namespace associated with this wrapping key.
    pub fn get_key_namespace(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_namespace
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn padding_scheme(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
        >,
    ) -> Self {
        self.padding_scheme = ::std::option::Option::Some(input.into());
        self
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn set_padding_scheme(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
        >,
    ) -> Self {
        self.padding_scheme = input;
        self
    }
    /// The RSA padding scheme to use with this keyring.
    pub fn get_padding_scheme(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::PaddingScheme>
    {
        &self.padding_scheme
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.private_key = ::std::option::Option::Some(input.into());
        self
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.private_key = input;
        self
    }
    /// The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 PrivateKeyInfo structure. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub fn get_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.private_key
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = input;
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`CreateRawRsaKeyringInput`](crate::operation::operation::CreateRawRsaKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput {
            key_name: self.key_name,
key_namespace: self.key_namespace,
padding_scheme: self.padding_scheme,
private_key: self.private_key,
public_key: self.public_key,
        })
    }
}
