// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::_generate_rsa_key_pair_output::GenerateRsaKeyPairOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::_generate_rsa_key_pair_input::GenerateRsaKeyPairInputBuilder;

impl GenerateRsaKeyPairInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        let mut fluent_builder = client.generate_rsa_key_pair();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateRsaKeyPair`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateRsaKeyPairFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairInputBuilder,
}
impl GenerateRsaKeyPairFluentBuilder {
    /// Creates a new `GenerateRsaKeyPair`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateRsaKeyPair as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput,
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
        crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPair::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn length_bits(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.inner = self.inner.length_bits(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_length_bits(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.inner = self.inner.set_length_bits(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_length_bits(&self) -> &::std::option::Option<::std::primitive::i32> {
        self.inner.get_length_bits()
    }
}
