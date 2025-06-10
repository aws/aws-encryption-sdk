// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::_unit::UnitBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::_decryption_materials::DecryptionMaterialsBuilder;

impl DecryptionMaterialsBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.decryption_materials_with_plaintext_data_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DecryptionMaterialsWithPlaintextDataKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DecryptionMaterialsWithPlaintextDataKeyFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsBuilder,
}
impl DecryptionMaterialsWithPlaintextDataKeyFluentBuilder {
    /// Creates a new `DecryptionMaterialsWithPlaintextDataKey`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DecryptionMaterialsWithPlaintextDataKey as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterialsWithPlaintextDataKey::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn algorithm_suite(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
        >,
    ) -> Self {
        self.inner = self.inner.algorithm_suite(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_algorithm_suite(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
        >,
    ) -> Self {
        self.inner = self.inner.set_algorithm_suite(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_algorithm_suite(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    > {
        self.inner.get_algorithm_suite()
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.encryption_context(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_encryption_context(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.inner.get_encryption_context()
    }
    #[allow(missing_docs)]
    pub fn plaintext_data_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.plaintext_data_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_plaintext_data_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_plaintext_data_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_plaintext_data_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_plaintext_data_key()
    }
    #[allow(missing_docs)]
    pub fn required_encryption_context_keys(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.required_encryption_context_keys(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_required_encryption_context_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_required_encryption_context_keys(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_required_encryption_context_keys()
    }
    #[allow(missing_docs)]
    pub fn symmetric_signing_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.symmetric_signing_key(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_symmetric_signing_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_symmetric_signing_key(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_symmetric_signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_symmetric_signing_key()
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
