// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating an Required Encryption Context Cryptographic Materials Manager.
pub struct CreateRequiredEncryptionContextCmmInput {
    /// The Keyring that the created Cryprographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryprographic Materials Manager must be specified as input.
pub keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
/// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
pub required_encryption_context_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
/// The Cryprographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryprographic Materials Manager must be specified.
pub underlying_cmm: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateRequiredEncryptionContextCmmInput {
    /// The Keyring that the created Cryprographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryprographic Materials Manager must be specified as input.
    pub fn keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.required_encryption_context_keys
    }
    /// The Cryprographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryprographic Materials Manager must be specified.
    pub fn underlying_cmm(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.underlying_cmm
    }
}
impl CreateRequiredEncryptionContextCmmInput {
    /// Creates a new builder-style object to manufacture [`CreateRequiredEncryptionContextCmmInput`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder::default()
    }
}

/// A builder for [`CreateRequiredEncryptionContextCmmInput`](crate::operation::operation::CreateRequiredEncryptionContextCmmInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRequiredEncryptionContextCmmInputBuilder {
    pub(crate) keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
pub(crate) required_encryption_context_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) underlying_cmm: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl CreateRequiredEncryptionContextCmmInputBuilder {
    /// The Keyring that the created Cryprographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryprographic Materials Manager must be specified as input.
    pub fn keyring(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = ::std::option::Option::Some(input.into());
        self
    }
    /// The Keyring that the created Cryprographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryprographic Materials Manager must be specified as input.
    pub fn set_keyring(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = input;
        self
    }
    /// The Keyring that the created Cryprographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryprographic Materials Manager must be specified as input.
    pub fn get_keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn required_encryption_context_keys(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.required_encryption_context_keys = ::std::option::Option::Some(input.into());
        self
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn set_required_encryption_context_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.required_encryption_context_keys = input;
        self
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn get_required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.required_encryption_context_keys
    }
    /// The Cryprographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryprographic Materials Manager must be specified.
    pub fn underlying_cmm(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.underlying_cmm = ::std::option::Option::Some(input.into());
        self
    }
    /// The Cryprographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryprographic Materials Manager must be specified.
    pub fn set_underlying_cmm(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.underlying_cmm = input;
        self
    }
    /// The Cryprographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryprographic Materials Manager must be specified.
    pub fn get_underlying_cmm(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        &self.underlying_cmm
    }
    /// Consumes the builder and constructs a [`CreateRequiredEncryptionContextCmmInput`](crate::operation::operation::CreateRequiredEncryptionContextCmmInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput {
            keyring: self.keyring,
required_encryption_context_keys: self.required_encryption_context_keys,
underlying_cmm: self.underlying_cmm,
        })
    }
}
