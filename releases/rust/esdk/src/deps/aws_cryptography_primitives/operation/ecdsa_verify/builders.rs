// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::_ecdsa_verify_output::EcdsaVerifyOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::_ecdsa_verify_input::EcdsaVerifyInputBuilder;

impl EcdsaVerifyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        ::std::primitive::bool,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let mut fluent_builder = client.ecdsa_verify();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EcdsaVerify`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EcdsaVerifyFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyInputBuilder,
}
impl EcdsaVerifyFluentBuilder {
    /// Creates a new `EcdsaVerify`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the EcdsaVerify as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        ::std::primitive::bool,
        crate::deps::aws_cryptography_primitives::types::error::Error,
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
                crate::deps::aws_cryptography_primitives::types::error::Error::OpaqueWithText {
                    obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
                    objMessage: msg,
                }
            })?;
        crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerify::send(
            &self.client,
            input,
        )
        .await
    }

    #[allow(missing_docs)]
    pub fn message(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.message(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_message(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_message()
    }
    #[allow(missing_docs)]
    pub fn signature(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.signature(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_signature(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_signature()
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
    #[allow(missing_docs)]
    pub fn verification_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.verification_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_verification_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_verification_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_verification_key()
    }
}
