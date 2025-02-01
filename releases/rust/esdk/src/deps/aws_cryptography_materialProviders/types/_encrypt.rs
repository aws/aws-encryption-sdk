// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub enum Encrypt {
    #[allow(missing_docs)]
    AesGcm(crate::deps::aws_cryptography_primitives::types::AesGcm),
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
impl Encrypt {
    /// Tries to convert the enum instance into [`AesGcm`](crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm), extracting the inner [`crate::deps::aws_cryptography_primitives::types::AesGcm`](crate::deps::aws_cryptography_primitives::types::AesGcm).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_aes_gcm(
        &self,
    ) -> ::std::result::Result<&crate::deps::aws_cryptography_primitives::types::AesGcm, &Self>
    {
        if let crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm(val) = &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`AesGcm`](crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm).
    pub fn is_aes_gcm(&self) -> ::std::primitive::bool {
        self.as_aes_gcm().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
