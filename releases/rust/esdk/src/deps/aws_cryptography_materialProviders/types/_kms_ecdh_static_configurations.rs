// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Allowed configurations when using KmsEcdhStaticConfigurations.
pub enum KmsEcdhStaticConfigurations {
    #[allow(missing_docs)]
    KmsPublicKeyDiscovery(
        crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput,
    ),
    #[allow(missing_docs)]
    KmsPrivateKeyToStaticPublicKey(
        crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput,
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
impl KmsEcdhStaticConfigurations {
    /// Tries to convert the enum instance into [`KmsPublicKeyDiscovery`](crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_kms_public_key_discovery(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Tries to convert the enum instance into [`KmsPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_kms_private_key_to_static_public_key(&self) -> ::std::result::Result<&crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput, &Self>{
        if let crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Returns true if this is a [`KmsPublicKeyDiscovery`](crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery).
    pub fn is_kms_public_key_discovery(&self) -> ::std::primitive::bool {
        self.as_kms_public_key_discovery().is_ok()
    }
    /// Returns true if this is a [`KmsPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey).
    pub fn is_kms_private_key_to_static_public_key(&self) -> ::std::primitive::bool {
        self.as_kms_private_key_to_static_public_key().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
