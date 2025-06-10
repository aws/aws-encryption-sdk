// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecryptionMaterials {
    #[allow(missing_docs)]
    pub algorithm_suite: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
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
    pub symmetric_signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptionMaterials {
    #[allow(missing_docs)]
    pub fn algorithm_suite(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    > {
        &self.algorithm_suite
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
    pub fn symmetric_signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.symmetric_signing_key
    }
    #[allow(missing_docs)]
    pub fn verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.verification_key
    }
}
impl DecryptionMaterials {
    /// Creates a new builder-style object to manufacture [`DecryptionMaterials`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterials).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsBuilder::default()
    }
}

/// A builder for [`DecryptionMaterials`](crate::operation::operation::DecryptionMaterials).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptionMaterialsBuilder {
    pub(crate) algorithm_suite: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    >,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) plaintext_data_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) required_encryption_context_keys:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) symmetric_signing_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) verification_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptionMaterialsBuilder {
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
    pub fn symmetric_signing_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.symmetric_signing_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_symmetric_signing_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.symmetric_signing_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_symmetric_signing_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.symmetric_signing_key
    }
    #[allow(missing_docs)]
    pub fn verification_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.verification_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_verification_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.verification_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_verification_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.verification_key
    }
    /// Consumes the builder and constructs a [`DecryptionMaterials`](crate::operation::operation::DecryptionMaterials).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterials,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterials {
            algorithm_suite: self.algorithm_suite,
encryption_context: self.encryption_context,
plaintext_data_key: self.plaintext_data_key,
required_encryption_context_keys: self.required_encryption_context_keys,
symmetric_signing_key: self.symmetric_signing_key,
verification_key: self.verification_key,
        })
    }
}
