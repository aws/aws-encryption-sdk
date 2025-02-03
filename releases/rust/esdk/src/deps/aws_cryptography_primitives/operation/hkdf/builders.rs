// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::hkdf::_hkdf_output::HkdfOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::hkdf::_hkdf_input::HkdfInputBuilder;

impl HkdfInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let mut fluent_builder = client.hkdf();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Hkdf`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct HkdfFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner:
        crate::deps::aws_cryptography_primitives::operation::hkdf::builders::HkdfInputBuilder,
}
impl HkdfFluentBuilder {
    /// Creates a new `Hkdf`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the Hkdf as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::deps::aws_cryptography_primitives::operation::hkdf::builders::HkdfInputBuilder
    {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
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
        crate::deps::aws_cryptography_primitives::operation::hkdf::Hkdf::send(&self.client, input)
            .await
    }

    #[allow(missing_docs)]
    pub fn digest_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.digest_algorithm(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_digest_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_digest_algorithm(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        self.inner.get_digest_algorithm()
    }
    #[allow(missing_docs)]
    pub fn expected_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.inner = self.inner.expected_length(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_expected_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.inner = self.inner.set_expected_length(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_expected_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        self.inner.get_expected_length()
    }
    #[allow(missing_docs)]
    pub fn ikm(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.ikm(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ikm(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_ikm(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_ikm()
    }
    #[allow(missing_docs)]
    pub fn info(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.info(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_info(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_info(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_info()
    }
    #[allow(missing_docs)]
    pub fn salt(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.salt(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_salt(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_salt(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_salt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_salt()
    }
}
