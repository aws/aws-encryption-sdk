// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsRsaKeyring`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encryption_algorithm(impl Into<Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::encryption_algorithm) / [`set_encryption_algorithm(Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::set_encryption_algorithm): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_client(impl Into<Option<crate::deps::com_amazonaws_kms::client::Client>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::kms_client) / [`set_kms_client(Option<crate::deps::com_amazonaws_kms::client::Client>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::set_kms_client): (undocumented)<br>
    ///   - [`kms_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<::std::string::String>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::set_kms_key_id): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_rsa_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_rsa_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsRsaKeyringError>`](crate::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyringError)
    pub fn create_aws_kms_rsa_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringFluentBuilder::new(self.clone())
    }
}
