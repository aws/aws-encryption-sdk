// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EncryptionMaterials {
    #[allow(missing_docs)]
    pub algorithm_suite: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
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
    pub plaintext_data_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub required_encryption_context_keys:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[allow(missing_docs)]
    pub signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub symmetric_signing_keys: ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
}
impl EncryptionMaterials {
    #[allow(missing_docs)]
    pub fn algorithm_suite(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    > {
        &self.algorithm_suite
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
    pub fn plaintext_data_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext_data_key
    }
    #[allow(missing_docs)]
    pub fn required_encryption_context_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.required_encryption_context_keys
    }
    #[allow(missing_docs)]
    pub fn signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signing_key
    }
    #[allow(missing_docs)]
    pub fn symmetric_signing_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>> {
        &self.symmetric_signing_keys
    }
}
impl EncryptionMaterials {
    /// Creates a new builder-style object to manufacture [`EncryptionMaterials`](crate::operation::initialize_encryption_materials::builders::EncryptionMaterials).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::builders::EncryptionMaterialsBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::builders::EncryptionMaterialsBuilder::default()
    }
}

/// A builder for [`EncryptionMaterials`](crate::operation::operation::EncryptionMaterials).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptionMaterialsBuilder {
    pub(crate) algorithm_suite: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    >,
    pub(crate) encrypted_data_keys: ::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) plaintext_data_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) required_encryption_context_keys:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) symmetric_signing_keys:
        ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
}
impl EncryptionMaterialsBuilder {
    #[allow(missing_docs)]
    pub fn algorithm_suite(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
        >,
    ) -> Self {
        self.algorithm_suite = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_algorithm_suite(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
        >,
    ) -> Self {
        self.algorithm_suite = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_algorithm_suite(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    > {
        &self.algorithm_suite
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
    pub fn plaintext_data_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.plaintext_data_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_plaintext_data_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.plaintext_data_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_plaintext_data_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext_data_key
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
    #[allow(missing_docs)]
    pub fn signing_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.signing_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signing_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.signing_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.signing_key
    }
    #[allow(missing_docs)]
    pub fn symmetric_signing_keys(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::aws_smithy_types::Blob>>,
    ) -> Self {
        self.symmetric_signing_keys = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_symmetric_signing_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
    ) -> Self {
        self.symmetric_signing_keys = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_symmetric_signing_keys(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>> {
        &self.symmetric_signing_keys
    }
    /// Consumes the builder and constructs a [`EncryptionMaterials`](crate::operation::operation::EncryptionMaterials).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials {
            algorithm_suite: self.algorithm_suite,
encrypted_data_keys: self.encrypted_data_keys,
encryption_context: self.encryption_context,
plaintext_data_key: self.plaintext_data_key,
required_encryption_context_keys: self.required_encryption_context_keys,
signing_key: self.signing_key,
symmetric_signing_keys: self.symmetric_signing_keys,
        })
    }
}
