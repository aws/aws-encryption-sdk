// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CompressPublicKeyInput {
    #[allow(missing_docs)]
    pub ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    #[allow(missing_docs)]
    pub public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl CompressPublicKeyInput {
    #[allow(missing_docs)]
    pub fn ecc_curve(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.ecc_curve
    }
    #[allow(missing_docs)]
    pub fn public_key(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey> {
        &self.public_key
    }
}
impl CompressPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`CompressPublicKeyInput`](crate::operation::compress_public_key::builders::CompressPublicKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::builders::CompressPublicKeyInputBuilder::default()
    }
}

/// A builder for [`CompressPublicKeyInput`](crate::operation::operation::CompressPublicKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CompressPublicKeyInputBuilder {
    pub(crate) ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    pub(crate) public_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
}
impl CompressPublicKeyInputBuilder {
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
    /// Consumes the builder and constructs a [`CompressPublicKeyInput`](crate::operation::operation::CompressPublicKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput {
            ecc_curve: self.ecc_curve,
public_key: self.public_key,
        })
    }
}
