// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a raw ECDH Keyring.
pub struct CreateRawEcdhKeyringInput {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub key_agreement_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
    >,
    /// The the curve on which the points for the sender's private and recipient's public key lie.
    pub curve_spec:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
}
impl CreateRawEcdhKeyringInput {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn key_agreement_scheme(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
    > {
        &self.key_agreement_scheme
    }
    /// The the curve on which the points for the sender's private and recipient's public key lie.
    pub fn curve_spec(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.curve_spec
    }
}
impl CreateRawEcdhKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateRawEcdhKeyringInput`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateRawEcdhKeyringInput`](crate::operation::operation::CreateRawEcdhKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRawEcdhKeyringInputBuilder {
    pub(crate) key_agreement_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
    >,
    pub(crate) curve_spec:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
}
impl CreateRawEcdhKeyringInputBuilder {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn key_agreement_scheme(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.key_agreement_scheme = ::std::option::Option::Some(input.into());
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn set_key_agreement_scheme(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.key_agreement_scheme = input;
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn get_key_agreement_scheme(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
    > {
        &self.key_agreement_scheme
    }
    /// The the curve on which the points for the sender's private and recipient's public key lie.
    pub fn curve_spec(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    ) -> Self {
        self.curve_spec = ::std::option::Option::Some(input.into());
        self
    }
    /// The the curve on which the points for the sender's private and recipient's public key lie.
    pub fn set_curve_spec(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
        >,
    ) -> Self {
        self.curve_spec = input;
        self
    }
    /// The the curve on which the points for the sender's private and recipient's public key lie.
    pub fn get_curve_spec(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.curve_spec
    }
    /// Consumes the builder and constructs a [`CreateRawEcdhKeyringInput`](crate::operation::operation::CreateRawEcdhKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput {
            key_agreement_scheme: self.key_agreement_scheme,
curve_spec: self.curve_spec,
        })
    }
}
