// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`ValidateCommitmentPolicyOnDecrypt`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>>)`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder::algorithm) / [`set_algorithm(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>)`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder::set_algorithm): (undocumented)<br>
    ///   - [`commitment_policy(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>>)`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder::commitment_policy) / [`set_commitment_policy(Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>)`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder::set_commitment_policy): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::validate_commitment_policy_on_decrypt::Unit) with field(s):

    /// - On failure, responds with [`SdkError<ValidateCommitmentPolicyOnDecryptError>`](crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptError)
    pub fn validate_commitment_policy_on_decrypt(&self) -> crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptFluentBuilder::new(self.clone())
    }
}
