// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::_get_encryption_materials_output::GetEncryptionMaterialsOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::_get_encryption_materials_input::GetEncryptionMaterialsInputBuilder;

impl GetEncryptionMaterialsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        cryptographic_materials_manager: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let mut fluent_builder = cryptographic_materials_manager.get_encryption_materials();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEncryptionMaterials`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEncryptionMaterialsFluentBuilder {
    cryptographic_materials_manager: crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::builders::GetEncryptionMaterialsInputBuilder,
}
impl GetEncryptionMaterialsFluentBuilder {
    /// Creates a new `GetEncryptionMaterials`.
    pub(crate) fn new(
        cryptographic_materials_manager: crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    ) -> Self {
        Self {
            cryptographic_materials_manager,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetEncryptionMaterials as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::builders::GetEncryptionMaterialsInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
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
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterials::send(&self.cryptographic_materials_manager, input).await
    }

    #[allow(missing_docs)]
    pub fn algorithm_suite_id(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.algorithm_suite_id(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_algorithm_suite_id(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.set_algorithm_suite_id(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_algorithm_suite_id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        self.inner.get_algorithm_suite_id()
    }
    #[allow(missing_docs)]
    pub fn commitment_policy(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
        >,
    ) -> Self {
        self.inner = self.inner.commitment_policy(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_commitment_policy(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
        >,
    ) -> Self {
        self.inner = self.inner.set_commitment_policy(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_commitment_policy(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    > {
        self.inner.get_commitment_policy()
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
    pub fn max_plaintext_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i64>,
    ) -> Self {
        self.inner = self.inner.max_plaintext_length(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_max_plaintext_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i64>,
    ) -> Self {
        self.inner = self.inner.set_max_plaintext_length(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_max_plaintext_length(&self) -> &::std::option::Option<::std::primitive::i64> {
        self.inner.get_max_plaintext_length()
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
}
