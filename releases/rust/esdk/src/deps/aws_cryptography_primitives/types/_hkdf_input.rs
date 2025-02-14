// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HkdfInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub expected_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub info: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub salt: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfInput {
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
    pub fn info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.info
    }
    #[allow(missing_docs)]
    pub fn salt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.salt
    }
}
impl HkdfInput {
    /// Creates a new builder-style object to manufacture [`HkdfInput`](crate::deps::aws_cryptography_primitives::types::HkdfInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::HkdfInputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::HkdfInputBuilder::default()
    }
}

/// A builder for [`HkdfInput`](crate::deps::aws_cryptography_primitives::types::HkdfInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) expected_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) info: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) salt: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfInputBuilder {
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
    pub fn info(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.info = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_info(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.info = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.info
    }
    #[allow(missing_docs)]
    pub fn salt(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.salt = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_salt(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.salt = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_salt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.salt
    }
    /// Consumes the builder and constructs a [`HkdfInput`](crate::deps::aws_cryptography_primitives::types::HkdfInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::HkdfInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::types::HkdfInput {
            digest_algorithm: self.digest_algorithm,
            expected_length: self.expected_length,
            ikm: self.ikm,
            info: self.info,
            salt: self.salt,
        })
    }
}
