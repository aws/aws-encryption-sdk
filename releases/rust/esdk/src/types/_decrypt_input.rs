// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecryptInput {
    #[allow(missing_docs)]
pub ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)]
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)]
pub keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
#[allow(missing_docs)]
pub materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl DecryptInput {
    #[allow(missing_docs)]
pub fn ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext
}
#[allow(missing_docs)]
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)]
pub fn keyring(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef> {
    &self.keyring
}
#[allow(missing_docs)]
pub fn materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    &self.materials_manager
}
}
impl DecryptInput {
    /// Creates a new builder-style object to manufacture [`DecryptInput`](crate::types::DecryptInput).
    pub fn builder() -> crate::types::builders::DecryptInputBuilder {
        crate::types::builders::DecryptInputBuilder::default()
    }
}

/// A builder for [`DecryptInput`](crate::types::DecryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptInputBuilder {
    pub(crate) ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
pub(crate) materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
}
impl DecryptInputBuilder {
    #[allow(missing_docs)]
pub fn ciphertext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_ciphertext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext = input;
    self
}
#[allow(missing_docs)]
pub fn get_ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext
}
#[allow(missing_docs)]
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = input;
    self
}
#[allow(missing_docs)]
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)]
pub fn keyring(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>) -> Self {
    self.keyring = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_keyring(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>) -> Self {
    self.keyring = input;
    self
}
#[allow(missing_docs)]
pub fn get_keyring(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef> {
    &self.keyring
}
#[allow(missing_docs)]
pub fn materials_manager(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.materials_manager = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_materials_manager(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.materials_manager = input;
    self
}
#[allow(missing_docs)]
pub fn get_materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    &self.materials_manager
}
    /// Consumes the builder and constructs a [`DecryptInput`](crate::types::DecryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::DecryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::DecryptInput {
            ciphertext: self.ciphertext,
encryption_context: self.encryption_context,
keyring: self.keyring,
materials_manager: self.materials_manager,
        })
    }
}
