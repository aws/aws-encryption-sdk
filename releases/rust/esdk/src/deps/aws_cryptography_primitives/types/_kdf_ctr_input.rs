// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct KdfCtrInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub expected_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub nonce: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub purpose: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KdfCtrInput {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn expected_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.expected_length
    }
    #[allow(missing_docs)]
    pub fn ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ikm
    }
    #[allow(missing_docs)]
    pub fn nonce(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.nonce
    }
    #[allow(missing_docs)]
    pub fn purpose(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.purpose
    }
}
impl KdfCtrInput {
    /// Creates a new builder-style object to manufacture [`KdfCtrInput`](crate::deps::aws_cryptography_primitives::types::KdfCtrInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::KdfCtrInputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::KdfCtrInputBuilder::default()
    }
}

/// A builder for [`KdfCtrInput`](crate::deps::aws_cryptography_primitives::types::KdfCtrInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KdfCtrInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) expected_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) nonce: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) purpose: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KdfCtrInputBuilder {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.digest_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_digest_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.digest_algorithm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn expected_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.expected_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_expected_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.expected_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_expected_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.expected_length
    }
    #[allow(missing_docs)]
    pub fn ikm(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.ikm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ikm(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.ikm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ikm
    }
    #[allow(missing_docs)]
    pub fn nonce(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.nonce = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_nonce(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.nonce = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_nonce(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.nonce
    }
    #[allow(missing_docs)]
    pub fn purpose(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.purpose = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_purpose(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.purpose = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_purpose(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.purpose
    }
    /// Consumes the builder and constructs a [`KdfCtrInput`](crate::deps::aws_cryptography_primitives::types::KdfCtrInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::KdfCtrInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::KdfCtrInput {
                digest_algorithm: self.digest_algorithm,
                expected_length: self.expected_length,
                ikm: self.ikm,
                nonce: self.nonce,
                purpose: self.purpose,
            },
        )
    }
}
