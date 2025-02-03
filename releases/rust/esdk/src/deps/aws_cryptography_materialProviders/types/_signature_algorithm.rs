// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub enum SignatureAlgorithm {
    #[allow(missing_docs)]
    Ecdsa(crate::deps::aws_cryptography_materialProviders::types::Ecdsa),
    #[allow(missing_docs)]
    None(crate::deps::aws_cryptography_materialProviders::types::None),
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
impl SignatureAlgorithm {
    /// Tries to convert the enum instance into [`Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::Ecdsa), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::Ecdsa).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_ecdsa(
        &self,
    ) -> ::std::result::Result<&crate::deps::aws_cryptography_materialProviders::types::Ecdsa, &Self>
    {
        if let crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::Ecdsa(
            val,
        ) = &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Tries to convert the enum instance into [`None`](crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::None), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::None`](crate::deps::aws_cryptography_materialProviders::types::None).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_none(
        &self,
    ) -> ::std::result::Result<&crate::deps::aws_cryptography_materialProviders::types::None, &Self>
    {
        if let crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::None(
            val,
        ) = &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Ecdsa`](crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::Ecdsa).
    pub fn is_ecdsa(&self) -> ::std::primitive::bool {
        self.as_ecdsa().is_ok()
    }
    /// Returns true if this is a [`None`](crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::None).
    pub fn is_none(&self) -> ::std::primitive::bool {
        self.as_none().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
