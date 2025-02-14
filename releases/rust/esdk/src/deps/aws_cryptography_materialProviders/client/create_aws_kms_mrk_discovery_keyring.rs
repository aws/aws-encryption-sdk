// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsMrkDiscoveryKeyring`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`discovery_filter(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::discovery_filter) / [`set_discovery_filter(Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::set_discovery_filter): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_client(impl Into<Option<crate::deps::com_amazonaws_kms::client::Client>>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::kms_client) / [`set_kms_client(Option<crate::deps::com_amazonaws_kms::client::Client>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::set_kms_client): (undocumented)<br>
    ///   - [`region(impl Into<Option<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::region) / [`set_region(Option<::std::string::String>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::set_region): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_mrk_discovery_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_mrk_discovery_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsMrkDiscoveryKeyringError>`](crate::operation::create_aws_kms_mrk_discovery_keyring::CreateAwsKmsMrkDiscoveryKeyringError)
    pub fn create_aws_kms_mrk_discovery_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::builders::CreateAwsKmsMrkDiscoveryKeyringFluentBuilder::new(self.clone())
    }
}
