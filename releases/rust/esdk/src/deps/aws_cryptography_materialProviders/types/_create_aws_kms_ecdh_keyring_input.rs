// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating an AWS KMS ECDH Keyring.
pub struct CreateAwsKmsEcdhKeyringInput {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub key_agreement_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    >,
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub curve_spec:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The KMS Client this Keyring will use to call KMS.
    pub kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl CreateAwsKmsEcdhKeyringInput {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn key_agreement_scheme(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    > {
        &self.key_agreement_scheme
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn curve_spec(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.curve_spec
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn kms_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.kms_client
    }
}
impl CreateAwsKmsEcdhKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsEcdhKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsEcdhKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsEcdhKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsEcdhKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsEcdhKeyringInputBuilder {
    pub(crate) key_agreement_scheme: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    >,
    pub(crate) curve_spec:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl CreateAwsKmsEcdhKeyringInputBuilder {
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn key_agreement_scheme(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.key_agreement_scheme = ::std::option::Option::Some(input.into());
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn set_key_agreement_scheme(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.key_agreement_scheme = input;
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn get_key_agreement_scheme(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    > {
        &self.key_agreement_scheme
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn curve_spec(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    ) -> Self {
        self.curve_spec = ::std::option::Option::Some(input.into());
        self
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn set_curve_spec(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
        >,
    ) -> Self {
        self.curve_spec = input;
        self
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn get_curve_spec(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        &self.curve_spec
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = ::std::option::Option::Some(input.into());
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn kms_client(
        mut self,
        input: impl ::std::convert::Into<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.kms_client = ::std::option::Option::Some(input.into());
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn set_kms_client(
        mut self,
        input: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.kms_client = input;
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn get_kms_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.kms_client
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsEcdhKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput {
                key_agreement_scheme: self.key_agreement_scheme,
                curve_spec: self.curve_spec,
                grant_tokens: self.grant_tokens,
                kms_client: self.kms_client,
            },
        )
    }
}
