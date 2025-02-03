// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_output::GenerateEcdsaSignatureKeyOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_input::GenerateEcdsaSignatureKeyInputBuilder;

impl GenerateEcdsaSignatureKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        let mut fluent_builder = client.generate_ecdsa_signature_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateEcdsaSignatureKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateEcdsaSignatureKeyFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder,
}
impl GenerateEcdsaSignatureKeyFluentBuilder {
    /// Creates a new `GenerateEcdsaSignatureKey`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateEcdsaSignatureKey as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
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
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKey::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn signature_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.signature_algorithm(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_signature_algorithm(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature_algorithm(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    > {
        self.inner.get_signature_algorithm()
    }
}
