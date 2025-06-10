// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct Identity {}
impl Identity {}
impl Identity {
    /// Creates a new builder-style object to manufacture [`Identity`](crate::deps::aws_cryptography_materialProviders::types::Identity).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::IdentityBuilder {
        crate::deps::aws_cryptography_materialProviders::types::builders::IdentityBuilder::default()
    }
}

/// A builder for [`Identity`](crate::deps::aws_cryptography_materialProviders::types::Identity).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IdentityBuilder {}
impl IdentityBuilder {
    /// Consumes the builder and constructs a [`Identity`](crate::deps::aws_cryptography_materialProviders::types::Identity).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::Identity,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::Identity {},
        )
    }
}
