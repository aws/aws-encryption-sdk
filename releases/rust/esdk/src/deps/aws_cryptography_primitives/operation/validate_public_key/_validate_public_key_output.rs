// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct ValidatePublicKeyOutput {
    #[allow(missing_docs)]
    pub success: ::std::option::Option<::std::primitive::bool>,
}
impl ValidatePublicKeyOutput {
    #[allow(missing_docs)]
    pub fn success(&self) -> &::std::option::Option<::std::primitive::bool> {
        &self.success
    }
}
impl ValidatePublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`ValidatePublicKeyOutput`](crate::operation::validate_public_key::builders::ValidatePublicKeyOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::validate_public_key::builders::ValidatePublicKeyOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::validate_public_key::builders::ValidatePublicKeyOutputBuilder::default()
    }
}

/// A builder for [`ValidatePublicKeyOutput`](crate::operation::operation::ValidatePublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidatePublicKeyOutputBuilder {
    pub(crate) success: ::std::option::Option<::std::primitive::bool>,
}
impl ValidatePublicKeyOutputBuilder {
    #[allow(missing_docs)]
    pub fn success(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
        self.success = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_success(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
        self.success = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_success(&self) -> &::std::option::Option<::std::primitive::bool> {
        &self.success
    }
    /// Consumes the builder and constructs a [`ValidatePublicKeyOutput`](crate::operation::operation::ValidatePublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput {
            success: self.success,
        })
    }
}
