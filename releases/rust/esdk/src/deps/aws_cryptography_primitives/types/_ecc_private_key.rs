// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EccPrivateKey {
    #[allow(missing_docs)]
    pub pem: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EccPrivateKey {
    #[allow(missing_docs)]
    pub fn pem(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.pem
    }
}
impl EccPrivateKey {
    /// Creates a new builder-style object to manufacture [`EccPrivateKey`](crate::deps::aws_cryptography_primitives::types::EccPrivateKey).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::EccPrivateKeyBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::EccPrivateKeyBuilder::default()
    }
}

/// A builder for [`EccPrivateKey`](crate::deps::aws_cryptography_primitives::types::EccPrivateKey).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EccPrivateKeyBuilder {
    pub(crate) pem: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EccPrivateKeyBuilder {
    #[allow(missing_docs)]
    pub fn pem(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.pem = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_pem(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.pem = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_pem(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.pem
    }
    /// Consumes the builder and constructs a [`EccPrivateKey`](crate::deps::aws_cryptography_primitives::types::EccPrivateKey).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::EccPrivateKey,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::EccPrivateKey { pem: self.pem },
        )
    }
}
