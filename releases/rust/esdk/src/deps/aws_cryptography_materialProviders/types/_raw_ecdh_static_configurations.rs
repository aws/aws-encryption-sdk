// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// List of configurations when using RawEcdhStaticConfigurations.
pub enum RawEcdhStaticConfigurations {
    #[allow(missing_docs)]
PublicKeyDiscovery(crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput),
#[allow(missing_docs)]
RawPrivateKeyToStaticPublicKey(crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput),
#[allow(missing_docs)]
EphemeralPrivateKeyToStaticPublicKey(crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput),
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
impl RawEcdhStaticConfigurations {
    /// Tries to convert the enum instance into [`PublicKeyDiscovery`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput`](crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_public_key_discovery(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Tries to convert the enum instance into [`RawPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_raw_private_key_to_static_public_key(&self) -> ::std::result::Result<&crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput, &Self>{
        if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Tries to convert the enum instance into [`EphemeralPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput`](crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_ephemeral_private_key_to_static_public_key(&self) -> ::std::result::Result<&crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput, &Self>{
        if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Returns true if this is a [`PublicKeyDiscovery`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery).
    pub fn is_public_key_discovery(&self) -> ::std::primitive::bool {
        self.as_public_key_discovery().is_ok()
    }
    /// Returns true if this is a [`RawPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey).
    pub fn is_raw_private_key_to_static_public_key(&self) -> ::std::primitive::bool {
        self.as_raw_private_key_to_static_public_key().is_ok()
    }
    /// Returns true if this is a [`EphemeralPrivateKeyToStaticPublicKey`](crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey).
    pub fn is_ephemeral_private_key_to_static_public_key(&self) -> ::std::primitive::bool {
        self.as_ephemeral_private_key_to_static_public_key().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
