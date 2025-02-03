// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::_create_multi_keyring_input::CreateMultiKeyringInputBuilder;

impl CreateMultiKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_multi_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMultiKeyring`.
///
/// Creates a Multi-Keyring comprised of one or more other Keyrings.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMultiKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::builders::CreateMultiKeyringInputBuilder,
}
impl CreateMultiKeyringFluentBuilder {
    /// Creates a new `CreateMultiKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateMultiKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::builders::CreateMultiKeyringInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
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
        crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::CreateMultiKeyring::send(&self.client, input).await
    }

    /// A list of keyrings (other than the generator) responsible for wrapping and unwrapping the data key.
    pub fn child_keyrings(
        mut self,
        input: impl ::std::convert::Into<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.child_keyrings(input.into());
        self
    }
    /// A list of keyrings (other than the generator) responsible for wrapping and unwrapping the data key.
    pub fn set_child_keyrings(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_child_keyrings(input);
        self
    }
    /// A list of keyrings (other than the generator) responsible for wrapping and unwrapping the data key.
    pub fn get_child_keyrings(
        &self,
    ) -> &::std::option::Option<
        ::std::vec::Vec<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    > {
        self.inner.get_child_keyrings()
    }
    /// A keyring responsible for wrapping and unwrapping the data key. This is the first keyring that will be used to wrap the data key, and may be responsible for additionally generating the data key.
    pub fn generator(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.generator(input.into());
        self
    }
    /// A keyring responsible for wrapping and unwrapping the data key. This is the first keyring that will be used to wrap the data key, and may be responsible for additionally generating the data key.
    pub fn set_generator(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    ) -> Self {
        self.inner = self.inner.set_generator(input);
        self
    }
    /// A keyring responsible for wrapping and unwrapping the data key. This is the first keyring that will be used to wrap the data key, and may be responsible for additionally generating the data key.
    pub fn get_generator(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    > {
        self.inner.get_generator()
    }
}
