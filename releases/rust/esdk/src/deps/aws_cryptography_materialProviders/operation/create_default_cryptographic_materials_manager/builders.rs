// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::_create_cryptographic_materials_manager_output::CreateCryptographicMaterialsManagerOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::_create_default_cryptographic_materials_manager_input::CreateDefaultCryptographicMaterialsManagerInputBuilder;

impl CreateDefaultCryptographicMaterialsManagerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let mut fluent_builder = client.create_default_cryptographic_materials_manager();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDefaultCryptographicMaterialsManager`.
///
/// Creates a Default Cryptographic Materials Manager.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDefaultCryptographicMaterialsManagerFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerInputBuilder,
}
impl CreateDefaultCryptographicMaterialsManagerFluentBuilder {
    /// Creates a new `CreateDefaultCryptographicMaterialsManager`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateDefaultCryptographicMaterialsManager as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerInputBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManager::send(&self.client, input).await
    }

    /// The Keyring that the created Default Cryprographic Materials Manager will use to wrap data keys.
    pub fn keyring(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.keyring(input.into());
        self
    }
    /// The Keyring that the created Default Cryprographic Materials Manager will use to wrap data keys.
    pub fn set_keyring(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.set_keyring(input);
        self
    }
    /// The Keyring that the created Default Cryprographic Materials Manager will use to wrap data keys.
    pub fn get_keyring(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        self.inner.get_keyring()
    }
}
