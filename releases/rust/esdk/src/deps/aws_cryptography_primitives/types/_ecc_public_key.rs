// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EccPublicKey {
    #[allow(missing_docs)]
    pub der: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EccPublicKey {
    #[allow(missing_docs)]
    pub fn der(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.der
    }
}
impl EccPublicKey {
    /// Creates a new builder-style object to manufacture [`EccPublicKey`](crate::deps::aws_cryptography_primitives::types::EccPublicKey).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::EccPublicKeyBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::EccPublicKeyBuilder::default()
    }
}

/// A builder for [`EccPublicKey`](crate::deps::aws_cryptography_primitives::types::EccPublicKey).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EccPublicKeyBuilder {
    pub(crate) der: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EccPublicKeyBuilder {
    #[allow(missing_docs)]
    pub fn der(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.der = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_der(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.der = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_der(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.der
    }
    /// Consumes the builder and constructs a [`EccPublicKey`](crate::deps::aws_cryptography_primitives::types::EccPublicKey).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::EccPublicKey,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::EccPublicKey { der: self.der },
        )
    }
}
