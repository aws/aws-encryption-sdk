// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Supported configurations for the StaticConfiguration Key Agreement Scheme.
pub enum StaticConfigurations {
    #[allow(missing_docs)]
    AwsKmsEcdh(crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations),
    #[allow(missing_docs)]
    RawEcdh(crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations),
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
impl StaticConfigurations {
    /// Tries to convert the enum instance into [`AwsKmsEcdh`](crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::AwsKmsEcdh), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations`](crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_aws_kms_ecdh(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::AwsKmsEcdh(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Tries to convert the enum instance into [`RawEcdh`](crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::RawEcdh), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_raw_ecdh(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::RawEcdh(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Returns true if this is a [`AwsKmsEcdh`](crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::AwsKmsEcdh).
    pub fn is_aws_kms_ecdh(&self) -> ::std::primitive::bool {
        self.as_aws_kms_ecdh().is_ok()
    }
    /// Returns true if this is a [`RawEcdh`](crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::RawEcdh).
    pub fn is_raw_ecdh(&self) -> ::std::primitive::bool {
        self.as_raw_ecdh().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
