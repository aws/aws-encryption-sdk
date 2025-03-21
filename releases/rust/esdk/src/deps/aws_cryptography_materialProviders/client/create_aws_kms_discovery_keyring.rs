// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsDiscoveryKeyring`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`discovery_filter(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::discovery_filter) / [`set_discovery_filter(Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::set_discovery_filter): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_client(impl Into<Option<crate::deps::com_amazonaws_kms::client::Client>>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::kms_client) / [`set_kms_client(Option<crate::deps::com_amazonaws_kms::client::Client>)`](crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::set_kms_client): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_discovery_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_discovery_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsDiscoveryKeyringError>`](crate::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringError)
    pub fn create_aws_kms_discovery_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringFluentBuilder::new(self.clone())
    }
}
