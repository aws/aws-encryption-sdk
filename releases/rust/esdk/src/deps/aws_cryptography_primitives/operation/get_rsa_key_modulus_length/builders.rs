// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_output::GetRsaKeyModulusLengthOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::_get_rsa_key_modulus_length_input::GetRsaKeyModulusLengthInputBuilder;

impl GetRsaKeyModulusLengthInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        let mut fluent_builder = client.get_rsa_key_modulus_length();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRsaKeyModulusLength`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRsaKeyModulusLengthFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthInputBuilder,
}
impl GetRsaKeyModulusLengthFluentBuilder {
    /// Creates a new `GetRsaKeyModulusLength`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetRsaKeyModulusLength as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
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
                crate::deps::aws_cryptography_primitives::types::error::Error::OpaqueWithText {
                    obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
                    objMessage: msg,
                }
            })?;
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLength::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.public_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_public_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_public_key()
    }
}
