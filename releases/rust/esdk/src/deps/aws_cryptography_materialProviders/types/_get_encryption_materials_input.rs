// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetEncryptionMaterialsInput {
    #[allow(missing_docs)]
    pub algorithm_suite_id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    #[allow(missing_docs)]
    pub commitment_policy: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
    #[allow(missing_docs)]
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[allow(missing_docs)]
    pub max_plaintext_length: ::std::option::Option<::std::primitive::i64>,
    #[allow(missing_docs)]
    pub required_encryption_context_keys:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GetEncryptionMaterialsInput {
    #[allow(missing_docs)]
    pub fn algorithm_suite_id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        &self.algorithm_suite_id
    }
    #[allow(missing_docs)]
    pub fn commitment_policy(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    > {
        &self.commitment_policy
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    #[allow(missing_docs)]
    pub fn max_plaintext_length(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.max_plaintext_length
    }
    #[allow(missing_docs)]
    pub fn required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.required_encryption_context_keys
    }
}
impl GetEncryptionMaterialsInput {
    /// Creates a new builder-style object to manufacture [`GetEncryptionMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::GetEncryptionMaterialsInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::GetEncryptionMaterialsInputBuilder::default()
    }
}

/// A builder for [`GetEncryptionMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetEncryptionMaterialsInputBuilder {
    pub(crate) algorithm_suite_id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    pub(crate) commitment_policy: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) max_plaintext_length: ::std::option::Option<::std::primitive::i64>,
    pub(crate) required_encryption_context_keys:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl GetEncryptionMaterialsInputBuilder {
    #[allow(missing_docs)]
    pub fn algorithm_suite_id(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.algorithm_suite_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_algorithm_suite_id(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.algorithm_suite_id = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_algorithm_suite_id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        &self.algorithm_suite_id
    }
    #[allow(missing_docs)]
    pub fn commitment_policy(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
        >,
    ) -> Self {
        self.commitment_policy = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_commitment_policy(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
        >,
    ) -> Self {
        self.commitment_policy = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_commitment_policy(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    > {
        &self.commitment_policy
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    #[allow(missing_docs)]
    pub fn max_plaintext_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i64>,
    ) -> Self {
        self.max_plaintext_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_max_plaintext_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i64>,
    ) -> Self {
        self.max_plaintext_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_max_plaintext_length(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.max_plaintext_length
    }
    #[allow(missing_docs)]
    pub fn required_encryption_context_keys(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.required_encryption_context_keys = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_required_encryption_context_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.required_encryption_context_keys = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.required_encryption_context_keys
    }
    /// Consumes the builder and constructs a [`GetEncryptionMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput {
                algorithm_suite_id: self.algorithm_suite_id,
                commitment_policy: self.commitment_policy,
                encryption_context: self.encryption_context,
                max_plaintext_length: self.max_plaintext_length,
                required_encryption_context_keys: self.required_encryption_context_keys,
            },
        )
    }
}
