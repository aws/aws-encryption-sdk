// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::_create_aws_kms_ecdh_keyring_input::CreateAwsKmsEcdhKeyringInputBuilder;

impl CreateAwsKmsEcdhKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_ecdh_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsEcdhKeyring`.
///
/// Creates an AWS KMS ECDH Keyring, which wraps and unwraps data keys by deriving a shared data key from the established shared secret between parties through the ECDH protocol.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsEcdhKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringInputBuilder,
}
impl CreateAwsKmsEcdhKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsEcdhKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsEcdhKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
	     let msg = format!("{:?}", e);
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyring::send(&self.client, input).await
    }

    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn key_agreement_scheme(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.inner = self.inner.key_agreement_scheme(input.into());
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn set_key_agreement_scheme(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
        >,
    ) -> Self {
        self.inner = self.inner.set_key_agreement_scheme(input);
        self
    }
    /// The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub fn get_key_agreement_scheme(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    > {
        self.inner.get_key_agreement_scheme()
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn curve_spec(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
    ) -> Self {
        self.inner = self.inner.curve_spec(input.into());
        self
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn set_curve_spec(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
        >,
    ) -> Self {
        self.inner = self.inner.set_curve_spec(input);
        self
    }
    /// The named curve that corresponds to the curve on which the sender's private and recipient's public key lie.
    pub fn get_curve_spec(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>
    {
        self.inner.get_curve_spec()
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.grant_tokens(input.into());
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_grant_tokens(input);
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_grant_tokens()
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn kms_client(
        mut self,
        input: impl ::std::convert::Into<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.inner = self.inner.kms_client(input.into());
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn set_kms_client(
        mut self,
        input: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.inner = self.inner.set_kms_client(input);
        self
    }
    /// The KMS Client this Keyring will use to call KMS.
    pub fn get_kms_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        self.inner.get_kms_client()
    }
}
