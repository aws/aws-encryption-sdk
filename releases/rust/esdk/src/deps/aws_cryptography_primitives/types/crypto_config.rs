// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CryptoConfig {}
impl CryptoConfig {}
impl CryptoConfig {
    /// Creates a new builder-style object to manufacture [`CryptoConfig`](crate::deps::aws_cryptography_primitives::types::CryptoConfig).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfigBuilder {
        crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfigBuilder::default(
        )
    }
}

/// A builder for [`CryptoConfig`](crate::deps::aws_cryptography_primitives::types::CryptoConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CryptoConfigBuilder {}
impl CryptoConfigBuilder {
    /// Consumes the builder and constructs a [`CryptoConfig`](crate::deps::aws_cryptography_primitives::types::CryptoConfig).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig {},
        )
    }
}
