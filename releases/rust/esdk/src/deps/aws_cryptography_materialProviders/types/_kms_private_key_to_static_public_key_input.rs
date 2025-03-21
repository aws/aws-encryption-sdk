// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a KmsPrivateKeyToStaticPublicKey Configuration.
pub struct KmsPrivateKeyToStaticPublicKeyInput {
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
    /// AWS KMS Key Identifier belonging to the sender.
    pub sender_kms_identifier: ::std::option::Option<::std::string::String>,
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub sender_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KmsPrivateKeyToStaticPublicKeyInput {
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub fn recipient_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.recipient_public_key
    }
    /// AWS KMS Key Identifier belonging to the sender.
    pub fn sender_kms_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.sender_kms_identifier
    }
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub fn sender_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.sender_public_key
    }
}
impl KmsPrivateKeyToStaticPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`KmsPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::KmsPrivateKeyToStaticPublicKeyInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::KmsPrivateKeyToStaticPublicKeyInputBuilder::default()
    }
}

/// A builder for [`KmsPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KmsPrivateKeyToStaticPublicKeyInputBuilder {
    pub(crate) recipient_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) sender_kms_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) sender_public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KmsPrivateKeyToStaticPublicKeyInputBuilder {
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub fn recipient_public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.recipient_public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub fn set_recipient_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.recipient_public_key = input;
        self
    }
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub fn get_recipient_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.recipient_public_key
    }
    /// AWS KMS Key Identifier belonging to the sender.
    pub fn sender_kms_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sender_kms_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// AWS KMS Key Identifier belonging to the sender.
    pub fn set_sender_kms_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sender_kms_identifier = input;
        self
    }
    /// AWS KMS Key Identifier belonging to the sender.
    pub fn get_sender_kms_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.sender_kms_identifier
    }
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub fn sender_public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.sender_public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub fn set_sender_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.sender_public_key = input;
        self
    }
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub fn get_sender_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.sender_public_key
    }
    /// Consumes the builder and constructs a [`KmsPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput {
            recipient_public_key: self.recipient_public_key,
sender_kms_identifier: self.sender_kms_identifier,
sender_public_key: self.sender_public_key,
        })
    }
}
