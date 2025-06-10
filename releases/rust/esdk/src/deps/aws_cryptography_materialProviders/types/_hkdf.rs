// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct Hkdf {
    #[allow(missing_docs)]
    pub hmac:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub input_key_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub output_key_length: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub salt_length: ::std::option::Option<::std::primitive::i32>,
}
impl Hkdf {
    #[allow(missing_docs)]
    pub fn hmac(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.hmac
    }
    #[allow(missing_docs)]
    pub fn input_key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.input_key_length
    }
    #[allow(missing_docs)]
    pub fn output_key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.output_key_length
    }
    #[allow(missing_docs)]
    pub fn salt_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.salt_length
    }
}
impl Hkdf {
    /// Creates a new builder-style object to manufacture [`Hkdf`](crate::deps::aws_cryptography_materialProviders::types::Hkdf).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::HkdfBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::HkdfBuilder::default()
    }
}

/// A builder for [`Hkdf`](crate::deps::aws_cryptography_materialProviders::types::Hkdf).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfBuilder {
    pub(crate) hmac:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) input_key_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) output_key_length: ::std::option::Option<::std::primitive::i32>,
    pub(crate) salt_length: ::std::option::Option<::std::primitive::i32>,
}
impl HkdfBuilder {
    #[allow(missing_docs)]
    pub fn hmac(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.hmac = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_hmac(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.hmac = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_hmac(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.hmac
    }
    #[allow(missing_docs)]
    pub fn input_key_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.input_key_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_input_key_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.input_key_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_input_key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.input_key_length
    }
    #[allow(missing_docs)]
    pub fn output_key_length(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.output_key_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_output_key_length(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.output_key_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_output_key_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.output_key_length
    }
    #[allow(missing_docs)]
    pub fn salt_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.salt_length = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_salt_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.salt_length = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_salt_length(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.salt_length
    }
    /// Consumes the builder and constructs a [`Hkdf`](crate::deps::aws_cryptography_materialProviders::types::Hkdf).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::Hkdf,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::Hkdf {
                hmac: self.hmac,
                input_key_length: self.input_key_length,
                output_key_length: self.output_key_length,
                salt_length: self.salt_length,
            },
        )
    }
}
