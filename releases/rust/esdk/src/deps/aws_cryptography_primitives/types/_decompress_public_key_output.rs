// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct DecompressPublicKeyOutput {
    #[allow(missing_docs)]
    pub public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl DecompressPublicKeyOutput {
    #[allow(missing_docs)]
    pub fn public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        &self.public_key
    }
}
impl DecompressPublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`DecompressPublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::DecompressPublicKeyOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::DecompressPublicKeyOutputBuilder::default()
    }
}

/// A builder for [`DecompressPublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecompressPublicKeyOutputBuilder {
    pub(crate) public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl DecompressPublicKeyOutputBuilder {
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
    /// Consumes the builder and constructs a [`DecompressPublicKeyOutput`](crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput {
                public_key: self.public_key,
            },
        )
    }
}
