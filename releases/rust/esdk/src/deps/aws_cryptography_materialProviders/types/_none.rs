// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct None {}
impl None {}
impl None {
    /// Creates a new builder-style object to manufacture [`None`](crate::deps::aws_cryptography_materialProviders::types::None).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::NoneBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::NoneBuilder::default()
    }
}

/// A builder for [`None`](crate::deps::aws_cryptography_materialProviders::types::None).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NoneBuilder {}
impl NoneBuilder {
    /// Consumes the builder and constructs a [`None`](crate::deps::aws_cryptography_materialProviders::types::None).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::None,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::None {})
    }
}
