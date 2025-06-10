// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GenerateEccKeyPairOutput {
    #[allow(missing_docs)]
    pub ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    #[allow(missing_docs)]
    pub private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
    #[allow(missing_docs)]
    pub public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl GenerateEccKeyPairOutput {
    #[allow(missing_docs)]
    pub fn ecc_curve(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.ecc_curve
    }
    #[allow(missing_docs)]
    pub fn private_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>
    {
        &self.private_key
    }
    #[allow(missing_docs)]
    pub fn public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        &self.public_key
    }
}
impl GenerateEccKeyPairOutput {
    /// Creates a new builder-style object to manufacture [`GenerateEccKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::GenerateEccKeyPairOutputBuilder
    {
        crate::deps::aws_cryptography_primitives::types::builders::GenerateEccKeyPairOutputBuilder::default()
    }
}

/// A builder for [`GenerateEccKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEccKeyPairOutputBuilder {
    pub(crate) ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    pub(crate) private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
    pub(crate) public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl GenerateEccKeyPairOutputBuilder {
    #[allow(missing_docs)]
    pub fn ecc_curve(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    ) -> Self {
        self.ecc_curve = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ecc_curve(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
        >,
    ) -> Self {
        self.ecc_curve = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_ecc_curve(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.ecc_curve
    }
    #[allow(missing_docs)]
    pub fn private_key(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
    ) -> Self {
        self.private_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_private_key(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EccPrivateKey,
        >,
    ) -> Self {
        self.private_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_private_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>
    {
        &self.private_key
    }
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
    /// Consumes the builder and constructs a [`GenerateEccKeyPairOutput`](crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput {
                ecc_curve: self.ecc_curve,
                private_key: self.private_key,
                public_key: self.public_key,
            },
        )
    }
}
