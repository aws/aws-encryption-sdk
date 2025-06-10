// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateRsaKeyPairOutput {
    #[allow(missing_docs)]
    pub private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>,
    #[allow(missing_docs)]
    pub public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
}
impl GenerateRsaKeyPairOutput {
    #[allow(missing_docs)]
    pub fn private_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>
    {
        &self.private_key
    }
    #[allow(missing_docs)]
    pub fn public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey> {
        &self.public_key
    }
}
impl GenerateRsaKeyPairOutput {
    /// Creates a new builder-style object to manufacture [`GenerateRsaKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::GenerateRsaKeyPairOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::GenerateRsaKeyPairOutputBuilder::default()
    }
}

/// A builder for [`GenerateRsaKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRsaKeyPairOutputBuilder {
    pub(crate) private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>,
    pub(crate) public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
}
impl GenerateRsaKeyPairOutputBuilder {
    #[allow(missing_docs)]
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>,
    ) -> Self {
        self.private_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::RsaPrivateKey,
        >,
    ) -> Self {
        self.private_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_private_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>
    {
        &self.private_key
    }
    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
    ) -> Self {
        self.public_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`GenerateRsaKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput {
                private_key: self.private_key,
                public_key: self.public_key,
            },
        )
    }
}
