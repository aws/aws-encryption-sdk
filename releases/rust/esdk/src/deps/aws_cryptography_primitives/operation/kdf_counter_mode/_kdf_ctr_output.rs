// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct KdfCtrOutput {
    #[allow(missing_docs)]
    pub okm: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KdfCtrOutput {
    #[allow(missing_docs)]
    pub fn okm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.okm
    }
}
impl KdfCtrOutput {
    /// Creates a new builder-style object to manufacture [`KdfCtrOutput`](crate::operation::kdf_counter_mode::builders::KdfCtrOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::builders::KdfCtrOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::builders::KdfCtrOutputBuilder::default()
    }
}

/// A builder for [`KdfCtrOutput`](crate::operation::operation::KdfCtrOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KdfCtrOutputBuilder {
    pub(crate) okm: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl KdfCtrOutputBuilder {
    #[allow(missing_docs)]
    pub fn okm(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.okm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_okm(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.okm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_okm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.okm
    }
    /// Consumes the builder and constructs a [`KdfCtrOutput`](crate::operation::operation::KdfCtrOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::KdfCtrOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::KdfCtrOutput {
                okm: self.okm,
            },
        )
    }
}
