// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateRawEcdhKeyring`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_agreement_scheme(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations>>)`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder::key_agreement_scheme) / [`set_key_agreement_scheme(Option<crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations>)`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder::set_key_agreement_scheme): (undocumented)<br>
    ///   - [`curve_spec(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder::curve_spec) / [`set_curve_spec(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder::set_curve_spec): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_raw_ecdh_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_raw_ecdh_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateRawEcdhKeyringError>`](crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringError)
    pub fn create_raw_ecdh_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringFluentBuilder::new(self.clone())
    }
}
