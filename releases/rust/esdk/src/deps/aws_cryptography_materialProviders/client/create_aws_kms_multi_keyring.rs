// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsMultiKeyring`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_supplier(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::client_supplier) / [`set_client_supplier(Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::set_client_supplier): (undocumented)<br>
    ///   - [`generator(impl Into<Option<::std::string::String>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::generator) / [`set_generator(Option<::std::string::String>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::set_generator): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_key_ids(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::kms_key_ids) / [`set_kms_key_ids(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::set_kms_key_ids): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_multi_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_multi_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsMultiKeyringError>`](crate::operation::create_aws_kms_multi_keyring::CreateAwsKmsMultiKeyringError)
    pub fn create_aws_kms_multi_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::builders::CreateAwsKmsMultiKeyringFluentBuilder::new(self.clone())
    }
}
