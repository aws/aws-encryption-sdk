// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetRsaKeyModulusLengthInput {
    #[allow(missing_docs)]
    pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetRsaKeyModulusLengthInput {
    #[allow(missing_docs)]
    pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
}
impl GetRsaKeyModulusLengthInput {
    /// Creates a new builder-style object to manufacture [`GetRsaKeyModulusLengthInput`](crate::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthInputBuilder::default()
    }
}

/// A builder for [`GetRsaKeyModulusLengthInput`](crate::operation::operation::GetRsaKeyModulusLengthInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRsaKeyModulusLengthInputBuilder {
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetRsaKeyModulusLengthInputBuilder {
    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`GetRsaKeyModulusLengthInput`](crate::operation::operation::GetRsaKeyModulusLengthInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput {
            public_key: self.public_key,
        })
    }
}
