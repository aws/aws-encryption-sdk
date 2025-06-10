// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DigestOutput {
    #[allow(missing_docs)]
    pub digest: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DigestOutput {
    #[allow(missing_docs)]
    pub fn digest(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.digest
    }
}
impl DigestOutput {
    /// Creates a new builder-style object to manufacture [`DigestOutput`](crate::operation::digest::builders::DigestOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestOutputBuilder::default()
    }
}

/// A builder for [`DigestOutput`](crate::operation::operation::DigestOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DigestOutputBuilder {
    pub(crate) digest: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DigestOutputBuilder {
    #[allow(missing_docs)]
    pub fn digest(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.digest = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_digest(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.digest = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_digest(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.digest
    }
    /// Consumes the builder and constructs a [`DigestOutput`](crate::operation::operation::DigestOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::digest::DigestOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::digest::DigestOutput {
                digest: self.digest,
            },
        )
    }
}
