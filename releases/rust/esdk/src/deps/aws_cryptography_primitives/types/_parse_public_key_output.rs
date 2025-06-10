// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct ParsePublicKeyOutput {
    #[allow(missing_docs)]
    pub public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl ParsePublicKeyOutput {
    #[allow(missing_docs)]
    pub fn public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        &self.public_key
    }
}
impl ParsePublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`ParsePublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::ParsePublicKeyOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::ParsePublicKeyOutputBuilder::default()
    }
}

/// A builder for [`ParsePublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParsePublicKeyOutputBuilder {
    pub(crate) public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl ParsePublicKeyOutputBuilder {
    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
    ) -> Self {
        self.public_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`ParsePublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput {
                public_key: self.public_key,
            },
        )
    }
}
