// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::_unit::UnitBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::_algorithm_suite_info::AlgorithmSuiteInfoBuilder;

impl AlgorithmSuiteInfoBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.valid_algorithm_suite_info();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ValidAlgorithmSuiteInfo`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ValidAlgorithmSuiteInfoFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::builders::AlgorithmSuiteInfoBuilder,
}
impl ValidAlgorithmSuiteInfoFluentBuilder {
    /// Creates a new `ValidAlgorithmSuiteInfo`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ValidAlgorithmSuiteInfo as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::builders::AlgorithmSuiteInfoBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        (),
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
        crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::ValidAlgorithmSuiteInfo::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn binary_id(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.binary_id(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_binary_id(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_binary_id(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_binary_id(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_binary_id()
    }
    #[allow(missing_docs)]
    pub fn commitment(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.commitment(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_commitment(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_commitment(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_commitment(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        self.inner.get_commitment()
    }
    #[allow(missing_docs)]
    pub fn edk_wrapping(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.edk_wrapping(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_edk_wrapping(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_edk_wrapping(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_edk_wrapping(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    > {
        self.inner.get_edk_wrapping()
    }
    #[allow(missing_docs)]
    pub fn encrypt(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.inner = self.inner.encrypt(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encrypt(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.inner = self.inner.set_encrypt(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_encrypt(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>
    {
        self.inner.get_encrypt()
    }
    #[allow(missing_docs)]
    pub fn id(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_id(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        self.inner.get_id()
    }
    #[allow(missing_docs)]
    pub fn kdf(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.kdf(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_kdf(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_kdf(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        self.inner.get_kdf()
    }
    #[allow(missing_docs)]
    pub fn message_version(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.inner = self.inner.message_version(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_message_version(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.inner = self.inner.set_message_version(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_message_version(&self) -> &::std::option::Option<::std::primitive::i32> {
        self.inner.get_message_version()
    }
    #[allow(missing_docs)]
    pub fn signature(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.signature(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_signature(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    > {
        self.inner.get_signature()
    }
    #[allow(missing_docs)]
    pub fn symmetric_signature(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.symmetric_signature(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_symmetric_signature(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
        >,
    ) -> Self {
        self.inner = self.inner.set_symmetric_signature(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_symmetric_signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    > {
        self.inner.get_symmetric_signature()
    }
}
