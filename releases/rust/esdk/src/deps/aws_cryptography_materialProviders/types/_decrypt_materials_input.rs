// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecryptMaterialsInput {
    #[allow(missing_docs)]
    pub algorithm_suite_id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    #[allow(missing_docs)]
    pub commitment_policy: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
    #[allow(missing_docs)]
    pub encrypted_data_keys: ::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
    #[allow(missing_docs)]
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[allow(missing_docs)]
    pub reproduced_encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl DecryptMaterialsInput {
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
    pub fn encrypted_data_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    > {
        &self.encrypted_data_keys
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
    pub fn reproduced_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.reproduced_encryption_context
    }
}
impl DecryptMaterialsInput {
    /// Creates a new builder-style object to manufacture [`DecryptMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::DecryptMaterialsInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::DecryptMaterialsInputBuilder::default()
    }
}

/// A builder for [`DecryptMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptMaterialsInputBuilder {
    pub(crate) algorithm_suite_id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    pub(crate) commitment_policy: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
    pub(crate) encrypted_data_keys: ::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) reproduced_encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl DecryptMaterialsInputBuilder {
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
    pub fn encrypted_data_keys(
        mut self,
        input: impl ::std::convert::Into<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
            >,
        >,
    ) -> Self {
        self.encrypted_data_keys = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encrypted_data_keys(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
            >,
        >,
    ) -> Self {
        self.encrypted_data_keys = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encrypted_data_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    > {
        &self.encrypted_data_keys
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
    pub fn reproduced_encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.reproduced_encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_reproduced_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.reproduced_encryption_context = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_reproduced_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.reproduced_encryption_context
    }
    /// Consumes the builder and constructs a [`DecryptMaterialsInput`](crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput {
                algorithm_suite_id: self.algorithm_suite_id,
                commitment_policy: self.commitment_policy,
                encrypted_data_keys: self.encrypted_data_keys,
                encryption_context: self.encryption_context,
                reproduced_encryption_context: self.reproduced_encryption_context,
            },
        )
    }
}
