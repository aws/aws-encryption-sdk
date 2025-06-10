// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetPublicKeyFromPrivateKeyOutput {
    #[allow(missing_docs)]
    pub ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    #[allow(missing_docs)]
    pub private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
    #[allow(missing_docs)]
    pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetPublicKeyFromPrivateKeyOutput {
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
    pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
}
impl GetPublicKeyFromPrivateKeyOutput {
    /// Creates a new builder-style object to manufacture [`GetPublicKeyFromPrivateKeyOutput`](crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::types::builders::GetPublicKeyFromPrivateKeyOutputBuilder{
        crate::deps::aws_cryptography_primitives::types::builders::GetPublicKeyFromPrivateKeyOutputBuilder::default()
    }
}

/// A builder for [`GetPublicKeyFromPrivateKeyOutput`](crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPublicKeyFromPrivateKeyOutputBuilder {
    pub(crate) ecc_curve:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    pub(crate) private_key:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetPublicKeyFromPrivateKeyOutputBuilder {
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
    /// Consumes the builder and constructs a [`GetPublicKeyFromPrivateKeyOutput`](crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput {
                ecc_curve: self.ecc_curve,
                private_key: self.private_key,
                public_key: self.public_key,
            },
        )
    }
}
