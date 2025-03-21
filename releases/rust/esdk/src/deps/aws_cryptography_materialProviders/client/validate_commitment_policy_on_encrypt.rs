// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`ValidateCommitmentPolicyOnEncrypt`](crate::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>>)`](crate::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder::algorithm) / [`set_algorithm(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>)`](crate::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder::set_algorithm): (undocumented)<br>
    ///   - [`commitment_policy(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>>)`](crate::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder::commitment_policy) / [`set_commitment_policy(Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>)`](crate::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder::set_commitment_policy): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::validate_commitment_policy_on_encrypt::Unit) with field(s):

    /// - On failure, responds with [`SdkError<ValidateCommitmentPolicyOnEncryptError>`](crate::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptError)
    pub fn validate_commitment_policy_on_encrypt(&self) -> crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::builders::ValidateCommitmentPolicyOnEncryptFluentBuilder::new(self.clone())
    }
}
