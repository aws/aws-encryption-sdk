// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::_on_encrypt_output::OnEncryptOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::_on_encrypt_input::OnEncryptInputBuilder;

impl OnEncryptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        keyring: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = keyring.on_encrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `OnEncrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct OnEncryptFluentBuilder {
    keyring: crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::builders::OnEncryptInputBuilder,
}
impl OnEncryptFluentBuilder {
    /// Creates a new `OnEncrypt`.
    pub(crate) fn new(
        keyring: crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    ) -> Self {
        Self {
            keyring,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the OnEncrypt as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::builders::OnEncryptInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
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
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncrypt::send(
            &self.keyring,
            input,
        )
        .await
    }

    #[allow(missing_docs)]
    pub fn materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.inner = self.inner.materials(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
        >,
    ) -> Self {
        self.inner = self.inner.set_materials(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    > {
        self.inner.get_materials()
    }
}
