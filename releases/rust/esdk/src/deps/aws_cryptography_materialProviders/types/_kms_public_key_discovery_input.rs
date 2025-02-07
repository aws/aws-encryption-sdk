// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a KmsPublicKeyDiscovery Configuration. This is a DECRYPT ONLY configuration.
pub struct KmsPublicKeyDiscoveryInput {
    /// AWS KMS key identifier belonging to the recipient.
    pub recipient_kms_identifier: ::std::option::Option<::std::string::String>,
}
impl KmsPublicKeyDiscoveryInput {
    /// AWS KMS key identifier belonging to the recipient.
    pub fn recipient_kms_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.recipient_kms_identifier
    }
}
impl KmsPublicKeyDiscoveryInput {
    /// Creates a new builder-style object to manufacture [`KmsPublicKeyDiscoveryInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::KmsPublicKeyDiscoveryInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::KmsPublicKeyDiscoveryInputBuilder::default()
    }
}

/// A builder for [`KmsPublicKeyDiscoveryInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KmsPublicKeyDiscoveryInputBuilder {
    pub(crate) recipient_kms_identifier: ::std::option::Option<::std::string::String>,
}
impl KmsPublicKeyDiscoveryInputBuilder {
    /// AWS KMS key identifier belonging to the recipient.
    pub fn recipient_kms_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recipient_kms_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// AWS KMS key identifier belonging to the recipient.
    pub fn set_recipient_kms_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recipient_kms_identifier = input;
        self
    }
    /// AWS KMS key identifier belonging to the recipient.
    pub fn get_recipient_kms_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.recipient_kms_identifier
    }
    /// Consumes the builder and constructs a [`KmsPublicKeyDiscoveryInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput {
                recipient_kms_identifier: self.recipient_kms_identifier,
            },
        )
    }
}
