// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Nothing should ever be cached.
pub struct NoCache {}
impl NoCache {}
impl NoCache {
    /// Creates a new builder-style object to manufacture [`NoCache`](crate::deps::aws_cryptography_materialProviders::types::NoCache).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::NoCacheBuilder {
        crate::deps::aws_cryptography_materialProviders::types::builders::NoCacheBuilder::default()
    }
}

/// A builder for [`NoCache`](crate::deps::aws_cryptography_materialProviders::types::NoCache).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NoCacheBuilder {}
impl NoCacheBuilder {
    /// Consumes the builder and constructs a [`NoCache`](crate::deps::aws_cryptography_materialProviders::types::NoCache).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::NoCache,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::NoCache {},
        )
    }
}
