// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsDiscoveryMultiKeyring`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_supplier(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::client_supplier) / [`set_client_supplier(Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::set_client_supplier): (undocumented)<br>
    ///   - [`discovery_filter(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::discovery_filter) / [`set_discovery_filter(Option<crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::set_discovery_filter): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`regions(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::regions) / [`set_regions(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::set_regions): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_discovery_multi_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_discovery_multi_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsDiscoveryMultiKeyringError>`](crate::operation::create_aws_kms_discovery_multi_keyring::CreateAwsKmsDiscoveryMultiKeyringError)
    pub fn create_aws_kms_discovery_multi_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_multi_keyring::builders::CreateAwsKmsDiscoveryMultiKeyringFluentBuilder::new(self.clone())
    }
}
