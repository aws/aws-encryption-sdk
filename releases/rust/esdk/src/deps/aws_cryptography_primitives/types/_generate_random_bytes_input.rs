// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateRandomBytesInput {
    #[allow(missing_docs)]
    pub length: ::std::option::Option<::std::primitive::i32>,
}
impl GenerateRandomBytesInput {
    #[allow(missing_docs)]
    pub fn length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length
    }
}
impl GenerateRandomBytesInput {
    /// Creates a new builder-style object to manufacture [`GenerateRandomBytesInput`](crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::GenerateRandomBytesInputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::GenerateRandomBytesInputBuilder::default()
    }
}

/// A builder for [`GenerateRandomBytesInput`](crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRandomBytesInputBuilder {
    pub(crate) length: ::std::option::Option<::std::primitive::i32>,
}
impl GenerateRandomBytesInputBuilder {
    #[allow(missing_docs)]
    pub fn length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length
    }
    /// Consumes the builder and constructs a [`GenerateRandomBytesInput`](crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput {
                length: self.length,
            },
        )
    }
}
