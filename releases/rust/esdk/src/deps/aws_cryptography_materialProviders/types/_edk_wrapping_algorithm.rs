// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub enum EdkWrappingAlgorithm {
    #[allow(missing_docs)]
    DirectKeyWrapping(crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping),
    #[allow(missing_docs)]
    IntermediateKeyWrapping(
        crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping,
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
impl EdkWrappingAlgorithm {
    /// Tries to convert the enum instance into [`DirectKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_direct_key_wrapping(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Tries to convert the enum instance into [`IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_intermediate_key_wrapping(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
    }
    /// Returns true if this is a [`DirectKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping).
    pub fn is_direct_key_wrapping(&self) -> ::std::primitive::bool {
        self.as_direct_key_wrapping().is_ok()
    }
    /// Returns true if this is a [`IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping).
    pub fn is_intermediate_key_wrapping(&self) -> ::std::primitive::bool {
        self.as_intermediate_key_wrapping().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
