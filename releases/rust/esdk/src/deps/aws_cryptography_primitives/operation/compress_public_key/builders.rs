// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::compress_public_key::_compress_public_key_output::CompressPublicKeyOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::compress_public_key::_compress_public_key_input::CompressPublicKeyInputBuilder;

impl CompressPublicKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyOutput,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    >{
        let mut fluent_builder = client.compress_public_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CompressPublicKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CompressPublicKeyFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyInputBuilder,
}
impl CompressPublicKeyFluentBuilder {
    /// Creates a new `CompressPublicKey`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CompressPublicKey as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyOutput,
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
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKey::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn ecc_curve(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    ) -> Self {
        self.inner = self.inner.ecc_curve(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ecc_curve(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
        >,
    ) -> Self {
        self.inner = self.inner.set_ecc_curve(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_ecc_curve(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        self.inner.get_ecc_curve()
    }
    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
    ) -> Self {
        self.inner = self.inner.public_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
    ) -> Self {
        self.inner = self.inner.set_public_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        self.inner.get_public_key()
    }
}
