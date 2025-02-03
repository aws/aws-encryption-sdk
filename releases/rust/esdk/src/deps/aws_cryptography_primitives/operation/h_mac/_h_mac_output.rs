// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HMacOutput {
    #[allow(missing_docs)]
    pub digest: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HMacOutput {
    #[allow(missing_docs)]
    pub fn digest(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.digest
    }
}
impl HMacOutput {
    /// Creates a new builder-style object to manufacture [`HMacOutput`](crate::operation::h_mac::builders::HMacOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::operation::h_mac::builders::HMacOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::operation::h_mac::builders::HMacOutputBuilder::default()
    }
}

/// A builder for [`HMacOutput`](crate::operation::operation::HMacOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HMacOutputBuilder {
    pub(crate) digest: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HMacOutputBuilder {
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
    /// Consumes the builder and constructs a [`HMacOutput`](crate::operation::operation::HMacOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::h_mac::HMacOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::h_mac::HMacOutput {
                digest: self.digest,
            },
        )
    }
}
