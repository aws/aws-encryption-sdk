// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub enum AlgorithmSuiteId {
    #[allow(missing_docs)]
    Esdk(crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId),
    #[allow(missing_docs)]
    Dbe(crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId),
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
impl AlgorithmSuiteId {
    /// Tries to convert the enum instance into [`Esdk`](crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Esdk), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId`](crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_esdk(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Esdk(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Tries to convert the enum instance into [`Dbe`](crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Dbe), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId`](crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_dbe(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Dbe(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Esdk`](crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Esdk).
    pub fn is_esdk(&self) -> ::std::primitive::bool {
        self.as_esdk().is_ok()
    }
    /// Returns true if this is a [`Dbe`](crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Dbe).
    pub fn is_dbe(&self) -> ::std::primitive::bool {
        self.as_dbe().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
