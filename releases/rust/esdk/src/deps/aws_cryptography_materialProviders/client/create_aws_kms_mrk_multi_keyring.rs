// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsMrkMultiKeyring`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_supplier(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::client_supplier) / [`set_client_supplier(Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::set_client_supplier): (undocumented)<br>
    ///   - [`generator(impl Into<Option<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::generator) / [`set_generator(Option<::std::string::String>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::set_generator): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_key_ids(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::kms_key_ids) / [`set_kms_key_ids(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::set_kms_key_ids): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_mrk_multi_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_mrk_multi_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsMrkMultiKeyringError>`](crate::operation::create_aws_kms_mrk_multi_keyring::CreateAwsKmsMrkMultiKeyringError)
    pub fn create_aws_kms_mrk_multi_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_multi_keyring::builders::CreateAwsKmsMrkMultiKeyringFluentBuilder::new(self.clone())
    }
}
