// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct RsaDecryptOutput {
    #[allow(missing_docs)]
    pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaDecryptOutput {
    #[allow(missing_docs)]
    pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
}
impl RsaDecryptOutput {
    /// Creates a new builder-style object to manufacture [`RsaDecryptOutput`](crate::operation::rsa_decrypt::builders::RsaDecryptOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptOutputBuilder{
        crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::builders::RsaDecryptOutputBuilder::default()
    }
}

/// A builder for [`RsaDecryptOutput`](crate::operation::operation::RsaDecryptOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RsaDecryptOutputBuilder {
    pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaDecryptOutputBuilder {
    #[allow(missing_docs)]
    pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
    /// Consumes the builder and constructs a [`RsaDecryptOutput`](crate::operation::operation::RsaDecryptOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptOutput {
                plaintext: self.plaintext,
            },
        )
    }
}
