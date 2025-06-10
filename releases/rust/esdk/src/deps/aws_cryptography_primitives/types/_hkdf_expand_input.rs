// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HkdfExpandInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub expected_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub info: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub prk: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExpandInput {
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
    pub fn info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.info
    }
    #[allow(missing_docs)]
    pub fn prk(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.prk
    }
}
impl HkdfExpandInput {
    /// Creates a new builder-style object to manufacture [`HkdfExpandInput`](crate::deps::aws_cryptography_primitives::types::HkdfExpandInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::HkdfExpandInputBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::HkdfExpandInputBuilder::default()
    }
}

/// A builder for [`HkdfExpandInput`](crate::deps::aws_cryptography_primitives::types::HkdfExpandInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfExpandInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) expected_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) info: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) prk: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExpandInputBuilder {
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
    pub fn prk(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.prk = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_prk(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.prk = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_prk(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.prk
    }
    /// Consumes the builder and constructs a [`HkdfExpandInput`](crate::deps::aws_cryptography_primitives::types::HkdfExpandInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::HkdfExpandInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::HkdfExpandInput {
                digest_algorithm: self.digest_algorithm,
                expected_length: self.expected_length,
                info: self.info,
                prk: self.prk,
            },
        )
    }
}
