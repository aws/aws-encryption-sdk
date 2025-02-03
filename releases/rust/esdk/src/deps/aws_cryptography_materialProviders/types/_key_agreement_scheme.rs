// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Supported ECDH Key Agreement Schemes.
pub enum KeyAgreementScheme {
    #[allow(missing_docs)]
    StaticConfiguration(
        crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations,
    ),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl KeyAgreementScheme {
    /// Tries to convert the enum instance into [`StaticConfiguration`](crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme::StaticConfiguration), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations`](crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_static_configuration(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme::StaticConfiguration(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Returns true if this is a [`StaticConfiguration`](crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme::StaticConfiguration).
    pub fn is_static_configuration(&self) -> ::std::primitive::bool {
        self.as_static_configuration().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
