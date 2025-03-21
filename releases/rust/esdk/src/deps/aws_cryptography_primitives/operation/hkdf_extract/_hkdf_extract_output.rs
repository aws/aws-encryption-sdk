// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HkdfExtractOutput {
    #[allow(missing_docs)]
    pub prk: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExtractOutput {
    #[allow(missing_docs)]
    pub fn prk(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.prk
    }
}
impl HkdfExtractOutput {
    /// Creates a new builder-style object to manufacture [`HkdfExtractOutput`](crate::operation::hkdf_extract::builders::HkdfExtractOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractOutputBuilder::default()
    }
}

/// A builder for [`HkdfExtractOutput`](crate::operation::operation::HkdfExtractOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfExtractOutputBuilder {
    pub(crate) prk: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExtractOutputBuilder {
    #[allow(missing_docs)]
    pub fn prk(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.prk = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_prk(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.prk = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_prk(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.prk
    }
    /// Consumes the builder and constructs a [`HkdfExtractOutput`](crate::operation::operation::HkdfExtractOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractOutput {
                prk: self.prk,
            },
        )
    }
}
