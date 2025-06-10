// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::_create_aws_kms_rsa_keyring_input::CreateAwsKmsRsaKeyringInputBuilder;

impl CreateAwsKmsRsaKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_rsa_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsRsaKeyring`.
///
/// Creates an AWS KMS RSA Keyring, which wraps and unwraps data keys using a single asymmetric AWS KMS Key for RSA.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsRsaKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringInputBuilder,
}
impl CreateAwsKmsRsaKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsRsaKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsRsaKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringInputBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyring::send(&self.client, input).await
    }

    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn encryption_algorithm(
        mut self,
        input: impl ::std::convert::Into<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    ) -> Self {
        self.inner = self.inner.encryption_algorithm(input.into());
        self
    }
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn set_encryption_algorithm(
        mut self,
        input: ::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    ) -> Self {
        self.inner = self.inner.set_encryption_algorithm(input);
        self
    }
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn get_encryption_algorithm(
        &self,
    ) -> &::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec> {
        self.inner.get_encryption_algorithm()
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
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.public_key(input.into());
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_public_key(input);
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_public_key()
    }
}
