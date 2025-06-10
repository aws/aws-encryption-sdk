// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a AWS KMS RSA Keyring.
pub struct CreateAwsKmsRsaKeyringInput {
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub encryption_algorithm: ::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The KMS Client this Keyring will use to call KMS.
    pub kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CreateAwsKmsRsaKeyringInput {
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn encryption_algorithm(
        &self,
    ) -> &::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec> {
        &self.encryption_algorithm
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
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
}
impl CreateAwsKmsRsaKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsRsaKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsRsaKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsRsaKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsRsaKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsRsaKeyringInputBuilder {
    pub(crate) encryption_algorithm:
        ::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) kms_client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl CreateAwsKmsRsaKeyringInputBuilder {
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn encryption_algorithm(
        mut self,
        input: impl ::std::convert::Into<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    ) -> Self {
        self.encryption_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn set_encryption_algorithm(
        mut self,
        input: ::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
    ) -> Self {
        self.encryption_algorithm = input;
        self
    }
    /// The RSA algorithm used to wrap and unwrap data keys.
    pub fn get_encryption_algorithm(
        &self,
    ) -> &::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec> {
        &self.encryption_algorithm
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
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// The ARN for the asymmetric AWS KMS Key for RSA responsible for wrapping and unwrapping data keys.
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = input;
        self
    }
    /// The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 SubjectPublicKeyInfo structure. This should be the public key as exported from KMS. If not specified, this Keyring cannot be used on encrypt.
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsRsaKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput {
                encryption_algorithm: self.encryption_algorithm,
                grant_tokens: self.grant_tokens,
                kms_client: self.kms_client,
                kms_key_id: self.kms_key_id,
                public_key: self.public_key,
            },
        )
    }
}
