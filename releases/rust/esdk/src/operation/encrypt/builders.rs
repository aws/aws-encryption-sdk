// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::encrypt::_encrypt_output::EncryptOutputBuilder;

pub use crate::operation::encrypt::_encrypt_input::EncryptInputBuilder;

impl EncryptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::encrypt::EncryptOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.encrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Encrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EncryptFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::encrypt::builders::EncryptInputBuilder,
}
impl EncryptFluentBuilder {
    /// Creates a new `Encrypt`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the Encrypt as a reference.
    pub fn as_input(&self) -> &crate::operation::encrypt::builders::EncryptInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt::EncryptOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
	     let msg = format!("{:?}", e);
             crate::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any),
		objMessage: msg
             }})?;
        crate::operation::encrypt::Encrypt::send(&self.client, input).await
    }

    #[allow(missing_docs)]
pub fn algorithm_suite_id(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.inner = self.inner.algorithm_suite_id(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_algorithm_suite_id(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>) -> Self {
    self.inner = self.inner.set_algorithm_suite_id(input);
    self
}
#[allow(missing_docs)]
pub fn get_algorithm_suite_id(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId> {
    self.inner.get_algorithm_suite_id()
}
#[allow(missing_docs)]
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.encryption_context(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.set_encryption_context(input);
    self
}
#[allow(missing_docs)]
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    self.inner.get_encryption_context()
}
#[allow(missing_docs)]
pub fn frame_length(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.inner = self.inner.frame_length(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_frame_length(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.inner = self.inner.set_frame_length(input);
    self
}
#[allow(missing_docs)]
pub fn get_frame_length(&self) -> &::std::option::Option<::std::primitive::i64> {
    self.inner.get_frame_length()
}
#[allow(missing_docs)]
pub fn keyring(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.keyring(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_keyring(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.set_keyring(input);
    self
}
#[allow(missing_docs)]
pub fn get_keyring(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef> {
    self.inner.get_keyring()
}
#[allow(missing_docs)]
pub fn materials_manager(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.materials_manager(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_materials_manager(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.set_materials_manager(input);
    self
}
#[allow(missing_docs)]
pub fn get_materials_manager(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    self.inner.get_materials_manager()
}
#[allow(missing_docs)]
pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.plaintext(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_plaintext(input);
    self
}
#[allow(missing_docs)]
pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_plaintext()
}
}
