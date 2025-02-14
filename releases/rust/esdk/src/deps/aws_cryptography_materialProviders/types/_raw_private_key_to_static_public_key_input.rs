// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a RawPrivateKeyToStaticPublicKey Configuration.
pub struct RawPrivateKeyToStaticPublicKeyInput {
    /// The recipient's public key. MUST be DER encoded.
    pub recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
    /// The sender's private key. MUST be PEM encoded.
    pub sender_static_private_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RawPrivateKeyToStaticPublicKeyInput {
    /// The recipient's public key. MUST be DER encoded.
    pub fn recipient_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.recipient_public_key
    }
    /// The sender's private key. MUST be PEM encoded.
    pub fn sender_static_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.sender_static_private_key
    }
}
impl RawPrivateKeyToStaticPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`RawPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::RawPrivateKeyToStaticPublicKeyInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::RawPrivateKeyToStaticPublicKeyInputBuilder::default()
    }
}

/// A builder for [`RawPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RawPrivateKeyToStaticPublicKeyInputBuilder {
    pub(crate) recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) sender_static_private_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RawPrivateKeyToStaticPublicKeyInputBuilder {
    /// The recipient's public key. MUST be DER encoded.
    pub fn recipient_public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.recipient_public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// The recipient's public key. MUST be DER encoded.
    pub fn set_recipient_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.recipient_public_key = input;
        self
    }
    /// The recipient's public key. MUST be DER encoded.
    pub fn get_recipient_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.recipient_public_key
    }
    /// The sender's private key. MUST be PEM encoded.
    pub fn sender_static_private_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.sender_static_private_key = ::std::option::Option::Some(input.into());
        self
    }
    /// The sender's private key. MUST be PEM encoded.
    pub fn set_sender_static_private_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.sender_static_private_key = input;
        self
    }
    /// The sender's private key. MUST be PEM encoded.
    pub fn get_sender_static_private_key(
        &self,
    ) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.sender_static_private_key
    }
    /// Consumes the builder and constructs a [`RawPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput {
            recipient_public_key: self.recipient_public_key,
sender_static_private_key: self.sender_static_private_key,
        })
    }
}
