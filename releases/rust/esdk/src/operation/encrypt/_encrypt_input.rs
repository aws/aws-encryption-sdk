// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EncryptInput {
    #[allow(missing_docs)]
pub algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
#[allow(missing_docs)]
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)]
pub frame_length: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)]
pub keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
#[allow(missing_docs)]
pub materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
#[allow(missing_docs)]
pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EncryptInput {
    #[allow(missing_docs)]
pub fn algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    &self.algorithm_suite_id
}
#[allow(missing_docs)]
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)]
pub fn frame_length(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.frame_length
}
#[allow(missing_docs)]
pub fn keyring(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef> {
    &self.keyring
}
#[allow(missing_docs)]
pub fn materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    &self.materials_manager
}
#[allow(missing_docs)]
pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
}
impl EncryptInput {
    /// Creates a new builder-style object to manufacture [`EncryptInput`](crate::operation::encrypt::builders::EncryptInput).
    pub fn builder() -> crate::operation::encrypt::builders::EncryptInputBuilder {
        crate::operation::encrypt::builders::EncryptInputBuilder::default()
    }
}

/// A builder for [`EncryptInput`](crate::operation::operation::EncryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptInputBuilder {
    pub(crate) algorithm_suite_id: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) frame_length: ::std::option::Option<::std::primitive::i64>,
pub(crate) keyring: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>,
pub(crate) materials_manager: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EncryptInputBuilder {
    #[allow(missing_docs)]
pub fn algorithm_suite_id(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.algorithm_suite_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_algorithm_suite_id(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.algorithm_suite_id = input;
    self
}
#[allow(missing_docs)]
pub fn get_algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    &self.algorithm_suite_id
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
pub fn frame_length(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.frame_length = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_frame_length(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.frame_length = input;
    self
}
#[allow(missing_docs)]
pub fn get_frame_length(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.frame_length
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
#[allow(missing_docs)]
pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = input;
    self
}
#[allow(missing_docs)]
pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
    /// Consumes the builder and constructs a [`EncryptInput`](crate::operation::operation::EncryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt::EncryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::encrypt::EncryptInput {
            algorithm_suite_id: self.algorithm_suite_id,
encryption_context: self.encryption_context,
frame_length: self.frame_length,
keyring: self.keyring,
materials_manager: self.materials_manager,
plaintext: self.plaintext,
        })
    }
}
