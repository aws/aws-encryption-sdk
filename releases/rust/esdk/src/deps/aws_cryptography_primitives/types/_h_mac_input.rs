// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HMacInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HMacInput {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key
    }
    #[allow(missing_docs)]
    pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
}
impl HMacInput {
    /// Creates a new builder-style object to manufacture [`HMacInput`](crate::deps::aws_cryptography_primitives::types::HMacInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::HMacInputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::HMacInputBuilder::default()
    }
}

/// A builder for [`HMacInput`](crate::deps::aws_cryptography_primitives::types::HMacInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HMacInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HMacInputBuilder {
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
    pub fn key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key
    }
    #[allow(missing_docs)]
    pub fn message(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
    /// Consumes the builder and constructs a [`HMacInput`](crate::deps::aws_cryptography_primitives::types::HMacInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::HMacInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::types::HMacInput {
            digest_algorithm: self.digest_algorithm,
            key: self.key,
            message: self.message,
        })
    }
}
