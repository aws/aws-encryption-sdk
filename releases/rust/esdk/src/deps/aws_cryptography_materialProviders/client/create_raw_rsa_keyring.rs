// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateRawRsaKeyring`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::key_name) / [`set_key_name(Option<::std::string::String>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::set_key_name): (undocumented)<br>
    ///   - [`key_namespace(impl Into<Option<::std::string::String>>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::key_namespace) / [`set_key_namespace(Option<::std::string::String>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::set_key_namespace): (undocumented)<br>
    ///   - [`padding_scheme(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::PaddingScheme>>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::padding_scheme) / [`set_padding_scheme(Option<crate::deps::aws_cryptography_materialProviders::types::PaddingScheme>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::set_padding_scheme): (undocumented)<br>
    ///   - [`private_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::private_key) / [`set_private_key(Option<::aws_smithy_types::Blob>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::set_private_key): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_raw_rsa_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_raw_rsa_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateRawRsaKeyringError>`](crate::operation::create_raw_rsa_keyring::CreateRawRsaKeyringError)
    pub fn create_raw_rsa_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringFluentBuilder::new(self.clone())
    }
}
