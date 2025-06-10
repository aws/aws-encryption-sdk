// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::_unit::UnitBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::_validate_commitment_policy_on_encrypt_input::ValidateCommitmentPolicyOnEncryptInputBuilder;

impl ValidateCommitmentPolicyOnEncryptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.validate_commitment_policy_on_encrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ValidateCommitmentPolicyOnEncrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ValidateCommitmentPolicyOnEncryptFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptInputBuilder,
}
impl ValidateCommitmentPolicyOnEncryptFluentBuilder {
    /// Creates a new `ValidateCommitmentPolicyOnEncrypt`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ValidateCommitmentPolicyOnEncrypt as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptInputBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncrypt::send(&self.client, input).await
    }

    #[allow(missing_docs)]
    pub fn algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.algorithm(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.inner = self.inner.set_algorithm(input);
        self
    }
    #[allow(missing_docs)]
    pub fn get_algorithm(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        self.inner.get_algorithm()
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
}
