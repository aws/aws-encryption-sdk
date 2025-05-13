// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::_create_required_encryption_context_cmm_output::CreateRequiredEncryptionContextCmmOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::_create_required_encryption_context_cmm_input::CreateRequiredEncryptionContextCmmInputBuilder;

impl CreateRequiredEncryptionContextCmmInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let mut fluent_builder = client.create_required_encryption_context_cmm();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRequiredEncryptionContextCmm`.
///
/// Creates an Required Encryption Context Cryptographic Materials Manager.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRequiredEncryptionContextCmmFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder,
}
impl CreateRequiredEncryptionContextCmmFluentBuilder {
    /// Creates a new `CreateRequiredEncryptionContextCmm`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateRequiredEncryptionContextCmm as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
	     let msg = format!("{:?}", e);
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmm::send(&self.client, input).await
    }

    /// The Keyring that the created Cryptographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryptographic Materials Manager must be specified as input.
    pub fn keyring(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.keyring(input.into());
        self
    }
    /// The Keyring that the created Cryptographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryptographic Materials Manager must be specified as input.
    pub fn set_keyring(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.set_keyring(input);
        self
    }
    /// The Keyring that the created Cryptographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryptographic Materials Manager must be specified as input.
    pub fn get_keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        self.inner.get_keyring()
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn required_encryption_context_keys(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.required_encryption_context_keys(input.into());
        self
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn set_required_encryption_context_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_required_encryption_context_keys(input);
        self
    }
    /// A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub fn get_required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_required_encryption_context_keys()
    }
    /// The Cryptographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryptographic Materials Manager must be specified.
    pub fn underlying_cmm(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.inner = self.inner.underlying_cmm(input.into());
        self
    }
    /// The Cryptographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryptographic Materials Manager must be specified.
    pub fn set_underlying_cmm(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
    ) -> Self {
        self.inner = self.inner.set_underlying_cmm(input);
        self
    }
    /// The Cryptographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryptographic Materials Manager must be specified.
    pub fn get_underlying_cmm(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>{
        self.inner.get_underlying_cmm()
    }
}
