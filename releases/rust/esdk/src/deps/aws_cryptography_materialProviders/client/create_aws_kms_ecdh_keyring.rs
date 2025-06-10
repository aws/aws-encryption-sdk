// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsEcdhKeyring`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_agreement_scheme(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations>>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::key_agreement_scheme) / [`set_key_agreement_scheme(Option<crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::set_key_agreement_scheme): (undocumented)<br>
    ///   - [`curve_spec(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::curve_spec) / [`set_curve_spec(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::set_curve_spec): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_client(impl Into<Option<crate::deps::com_amazonaws_kms::client::Client>>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::kms_client) / [`set_kms_client(Option<crate::deps::com_amazonaws_kms::client::Client>)`](crate::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::set_kms_client): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_ecdh_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_ecdh_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsEcdhKeyringError>`](crate::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyringError)
    pub fn create_aws_kms_ecdh_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::builders::CreateAwsKmsEcdhKeyringFluentBuilder::new(self.clone())
    }
}
