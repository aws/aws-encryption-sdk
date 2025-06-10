// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetRsaKeyModulusLengthOutput {
    #[allow(missing_docs)]
    pub length: ::std::option::Option<::std::primitive::i32>,
}
impl GetRsaKeyModulusLengthOutput {
    #[allow(missing_docs)]
    pub fn length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.length
    }
}
impl GetRsaKeyModulusLengthOutput {
    /// Creates a new builder-style object to manufacture [`GetRsaKeyModulusLengthOutput`](crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::GetRsaKeyModulusLengthOutputBuilder{
        crate::deps::aws_cryptography_primitives::types::builders::GetRsaKeyModulusLengthOutputBuilder::default()
    }
}

/// A builder for [`GetRsaKeyModulusLengthOutput`](crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRsaKeyModulusLengthOutputBuilder {
    pub(crate) length: ::std::option::Option<::std::primitive::i32>,
}
impl GetRsaKeyModulusLengthOutputBuilder {
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
    /// Consumes the builder and constructs a [`GetRsaKeyModulusLengthOutput`](crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput {
                length: self.length,
            },
        )
    }
}
