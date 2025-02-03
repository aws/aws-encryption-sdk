// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for creating a Keyring.
pub struct CreateKeyringOutput {
    /// The created Keyring.
    pub keyring: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
}
impl CreateKeyringOutput {
    /// The created Keyring.
    pub fn keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
}
impl CreateKeyringOutput {
    /// Creates a new builder-style object to manufacture [`CreateKeyringOutput`](crate::operation::create_raw_ecdh_keyring::builders::CreateKeyringOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateKeyringOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateKeyringOutputBuilder::default()
    }
}

/// A builder for [`CreateKeyringOutput`](crate::operation::operation::CreateKeyringOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyringOutputBuilder {
    pub(crate) keyring: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
}
impl CreateKeyringOutputBuilder {
    /// The created Keyring.
    pub fn keyring(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = ::std::option::Option::Some(input.into());
        self
    }
    /// The created Keyring.
    pub fn set_keyring(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.keyring = input;
        self
    }
    /// The created Keyring.
    pub fn get_keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        &self.keyring
    }
    /// Consumes the builder and constructs a [`CreateKeyringOutput`](crate::operation::operation::CreateKeyringOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::CreateKeyringOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::CreateKeyringOutput {
            keyring: self.keyring,
        })
    }
}
