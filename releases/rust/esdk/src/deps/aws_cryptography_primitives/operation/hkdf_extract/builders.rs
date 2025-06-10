// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_primitives::operation::hkdf_extract::_hkdf_extract_output::HkdfExtractOutputBuilder;

pub use crate::deps::aws_cryptography_primitives::operation::hkdf_extract::_hkdf_extract_input::HkdfExtractInputBuilder;

impl HkdfExtractInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_primitives::client::Client,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::deps::aws_cryptography_primitives::types::error::Error,
    > {
        let mut fluent_builder = client.hkdf_extract();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `HkdfExtract`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct HkdfExtractFluentBuilder {
    client: crate::deps::aws_cryptography_primitives::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractInputBuilder,
}
impl HkdfExtractFluentBuilder {
    /// Creates a new `HkdfExtract`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the HkdfExtract as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractInputBuilder{
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
        crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtract::send(
            &self.client,
            input,
        )
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
