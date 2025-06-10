// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a Default Cryptographic Materials Manager.
pub struct CreateDefaultCryptographicMaterialsManagerInput {
    /// The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.
    pub keyring: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
}
impl CreateDefaultCryptographicMaterialsManagerInput {
    /// The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.
    pub fn keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
}
impl CreateDefaultCryptographicMaterialsManagerInput {
    /// Creates a new builder-style object to manufacture [`CreateDefaultCryptographicMaterialsManagerInput`](crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::CreateDefaultCryptographicMaterialsManagerInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::CreateDefaultCryptographicMaterialsManagerInputBuilder::default()
    }
}

/// A builder for [`CreateDefaultCryptographicMaterialsManagerInput`](crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDefaultCryptographicMaterialsManagerInputBuilder {
    pub(crate) keyring: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
}
impl CreateDefaultCryptographicMaterialsManagerInputBuilder {
    /// The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.
    pub fn keyring(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = ::std::option::Option::Some(input.into());
        self
    }
    /// The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.
    pub fn set_keyring(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = input;
        self
    }
    /// The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.
    pub fn get_keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
    /// Consumes the builder and constructs a [`CreateDefaultCryptographicMaterialsManagerInput`](crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput {
            keyring: self.keyring,
        })
    }
}
