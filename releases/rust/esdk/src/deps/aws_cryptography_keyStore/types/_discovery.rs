// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct Discovery {}
impl Discovery {}
impl Discovery {
    /// Creates a new builder-style object to manufacture [`Discovery`](crate::deps::aws_cryptography_keyStore::types::Discovery).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::types::builders::DiscoveryBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::DiscoveryBuilder::default()
    }
}

/// A builder for [`Discovery`](crate::deps::aws_cryptography_keyStore::types::Discovery).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DiscoveryBuilder {}
impl DiscoveryBuilder {
    /// Consumes the builder and constructs a [`Discovery`](crate::deps::aws_cryptography_keyStore::types::Discovery).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::Discovery,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_keyStore::types::Discovery {})
    }
}
