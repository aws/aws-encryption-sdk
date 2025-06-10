// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateRandomBytesOutput {
    #[allow(missing_docs)]
    pub data: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateRandomBytesOutput {
    #[allow(missing_docs)]
    pub fn data(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.data
    }
}
impl GenerateRandomBytesOutput {
    /// Creates a new builder-style object to manufacture [`GenerateRandomBytesOutput`](crate::operation::generate_random_bytes::builders::GenerateRandomBytesOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::builders::GenerateRandomBytesOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::builders::GenerateRandomBytesOutputBuilder::default()
    }
}

/// A builder for [`GenerateRandomBytesOutput`](crate::operation::operation::GenerateRandomBytesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRandomBytesOutputBuilder {
    pub(crate) data: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateRandomBytesOutputBuilder {
    #[allow(missing_docs)]
    pub fn data(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.data = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_data(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.data = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_data(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.data
    }
    /// Consumes the builder and constructs a [`GenerateRandomBytesOutput`](crate::operation::operation::GenerateRandomBytesOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesOutput {
            data: self.data,
        })
    }
}
