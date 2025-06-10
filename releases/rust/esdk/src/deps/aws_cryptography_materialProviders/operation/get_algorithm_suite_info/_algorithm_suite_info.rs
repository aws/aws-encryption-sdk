// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AlgorithmSuiteInfo {
    #[allow(missing_docs)]
    pub binary_id: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub commitment: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    #[allow(missing_docs)]
    pub edk_wrapping: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    >,
    #[allow(missing_docs)]
    pub encrypt:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
    #[allow(missing_docs)]
    pub id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    #[allow(missing_docs)]
    pub kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    #[allow(missing_docs)]
    pub message_version: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub signature: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    >,
    #[allow(missing_docs)]
    pub symmetric_signature: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    >,
}
impl AlgorithmSuiteInfo {
    #[allow(missing_docs)]
    pub fn binary_id(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.binary_id
    }
    #[allow(missing_docs)]
    pub fn commitment(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.commitment
    }
    #[allow(missing_docs)]
    pub fn edk_wrapping(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    > {
        &self.edk_wrapping
    }
    #[allow(missing_docs)]
    pub fn encrypt(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>
    {
        &self.encrypt
    }
    #[allow(missing_docs)]
    pub fn id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        &self.id
    }
    #[allow(missing_docs)]
    pub fn kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.kdf
    }
    #[allow(missing_docs)]
    pub fn message_version(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.message_version
    }
    #[allow(missing_docs)]
    pub fn signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    > {
        &self.signature
    }
    #[allow(missing_docs)]
    pub fn symmetric_signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    > {
        &self.symmetric_signature
    }
}
impl AlgorithmSuiteInfo {
    /// Creates a new builder-style object to manufacture [`AlgorithmSuiteInfo`](crate::operation::get_algorithm_suite_info::builders::AlgorithmSuiteInfo).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::builders::AlgorithmSuiteInfoBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::builders::AlgorithmSuiteInfoBuilder::default()
    }
}

/// A builder for [`AlgorithmSuiteInfo`](crate::operation::operation::AlgorithmSuiteInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AlgorithmSuiteInfoBuilder {
    pub(crate) binary_id: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) commitment: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    pub(crate) edk_wrapping: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    >,
    pub(crate) encrypt:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
    pub(crate) id: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
    pub(crate) kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    pub(crate) message_version: ::std::option::Option<::std::primitive::i32>,
    pub(crate) signature: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    >,
    pub(crate) symmetric_signature: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    >,
}
impl AlgorithmSuiteInfoBuilder {
    #[allow(missing_docs)]
    pub fn binary_id(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.binary_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_binary_id(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.binary_id = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_binary_id(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.binary_id
    }
    #[allow(missing_docs)]
    pub fn commitment(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.commitment = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_commitment(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.commitment = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_commitment(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.commitment
    }
    #[allow(missing_docs)]
    pub fn edk_wrapping(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
        >,
    ) -> Self {
        self.edk_wrapping = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_edk_wrapping(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
        >,
    ) -> Self {
        self.edk_wrapping = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_edk_wrapping(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    > {
        &self.edk_wrapping
    }
    #[allow(missing_docs)]
    pub fn encrypt(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.encrypt = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encrypt(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.encrypt = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encrypt(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>
    {
        &self.encrypt
    }
    #[allow(missing_docs)]
    pub fn id(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_id(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
        >,
    ) -> Self {
        self.id = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_id(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    > {
        &self.id
    }
    #[allow(missing_docs)]
    pub fn kdf(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.kdf = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_kdf(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.kdf = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.kdf
    }
    #[allow(missing_docs)]
    pub fn message_version(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.message_version = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_message_version(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.message_version = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_message_version(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.message_version
    }
    #[allow(missing_docs)]
    pub fn signature(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
        >,
    ) -> Self {
        self.signature = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_signature(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
        >,
    ) -> Self {
        self.signature = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    > {
        &self.signature
    }
    #[allow(missing_docs)]
    pub fn symmetric_signature(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
        >,
    ) -> Self {
        self.symmetric_signature = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_symmetric_signature(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
        >,
    ) -> Self {
        self.symmetric_signature = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_symmetric_signature(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    > {
        &self.symmetric_signature
    }
    /// Consumes the builder and constructs a [`AlgorithmSuiteInfo`](crate::operation::operation::AlgorithmSuiteInfo).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::AlgorithmSuiteInfo,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::AlgorithmSuiteInfo {
            binary_id: self.binary_id,
commitment: self.commitment,
edk_wrapping: self.edk_wrapping,
encrypt: self.encrypt,
id: self.id,
kdf: self.kdf,
message_version: self.message_version,
signature: self.signature,
symmetric_signature: self.symmetric_signature,
        })
    }
}
