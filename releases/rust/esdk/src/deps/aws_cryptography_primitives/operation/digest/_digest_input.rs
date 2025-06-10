// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DigestInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DigestInput {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.message
    }
}
impl DigestInput {
    /// Creates a new builder-style object to manufacture [`DigestInput`](crate::operation::digest::builders::DigestInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestInputBuilder
    {
        crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestInputBuilder::default()
    }
}

/// A builder for [`DigestInput`](crate::operation::operation::DigestInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DigestInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DigestInputBuilder {
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
    /// Consumes the builder and constructs a [`DigestInput`](crate::operation::operation::DigestInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::digest::DigestInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::digest::DigestInput {
                digest_algorithm: self.digest_algorithm,
                message: self.message,
            },
        )
    }
}
