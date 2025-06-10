// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub enum Materials {
    #[allow(missing_docs)]
    Encryption(crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials),
    #[allow(missing_docs)]
    Decryption(crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials),
    #[allow(missing_docs)]
    BranchKey(crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials),
    #[allow(missing_docs)]
    BeaconKey(crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials),
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
impl Materials {
    /// Tries to convert the enum instance into [`Encryption`](crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials`](crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_encryption(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Tries to convert the enum instance into [`Decryption`](crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption), extracting the inner [`crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials`](crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_decryption(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Tries to convert the enum instance into [`BranchKey`](crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey), extracting the inner [`crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_branch_key(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Tries to convert the enum instance into [`BeaconKey`](crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey), extracting the inner [`crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_beacon_key(
        &self,
    ) -> ::std::result::Result<
        &crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
        &Self,
    > {
        if let crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey(val) =
            &self
        {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Encryption`](crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption).
    pub fn is_encryption(&self) -> ::std::primitive::bool {
        self.as_encryption().is_ok()
    }
    /// Returns true if this is a [`Decryption`](crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption).
    pub fn is_decryption(&self) -> ::std::primitive::bool {
        self.as_decryption().is_ok()
    }
    /// Returns true if this is a [`BranchKey`](crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey).
    pub fn is_branch_key(&self) -> ::std::primitive::bool {
        self.as_branch_key().is_ok()
    }
    /// Returns true if this is a [`BeaconKey`](crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey).
    pub fn is_beacon_key(&self) -> ::std::primitive::bool {
        self.as_beacon_key().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
