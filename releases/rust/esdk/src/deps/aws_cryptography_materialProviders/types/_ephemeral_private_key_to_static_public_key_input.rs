// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a EphemeralPrivateKeyToStaticPublicKey Configuration.
pub struct EphemeralPrivateKeyToStaticPublicKeyInput {
    /// The recipient's public key. MUST be DER encoded.
    pub recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EphemeralPrivateKeyToStaticPublicKeyInput {
    /// The recipient's public key. MUST be DER encoded.
    pub fn recipient_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.recipient_public_key
    }
}
impl EphemeralPrivateKeyToStaticPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`EphemeralPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::EphemeralPrivateKeyToStaticPublicKeyInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::EphemeralPrivateKeyToStaticPublicKeyInputBuilder::default()
    }
}

/// A builder for [`EphemeralPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EphemeralPrivateKeyToStaticPublicKeyInputBuilder {
    pub(crate) recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EphemeralPrivateKeyToStaticPublicKeyInputBuilder {
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
    /// Consumes the builder and constructs a [`EphemeralPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput {
            recipient_public_key: self.recipient_public_key,
        })
    }
}
