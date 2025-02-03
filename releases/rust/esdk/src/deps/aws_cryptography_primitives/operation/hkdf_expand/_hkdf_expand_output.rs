// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HkdfExpandOutput {
    #[allow(missing_docs)]
    pub okm: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExpandOutput {
    #[allow(missing_docs)]
    pub fn okm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.okm
    }
}
impl HkdfExpandOutput {
    /// Creates a new builder-style object to manufacture [`HkdfExpandOutput`](crate::operation::hkdf_expand::builders::HkdfExpandOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::hkdf_expand::builders::HkdfExpandOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::hkdf_expand::builders::HkdfExpandOutputBuilder::default()
    }
}

/// A builder for [`HkdfExpandOutput`](crate::operation::operation::HkdfExpandOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfExpandOutputBuilder {
    pub(crate) okm: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExpandOutputBuilder {
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
    /// Consumes the builder and constructs a [`HkdfExpandOutput`](crate::operation::operation::HkdfExpandOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandOutput {
                okm: self.okm,
            },
        )
    }
}
