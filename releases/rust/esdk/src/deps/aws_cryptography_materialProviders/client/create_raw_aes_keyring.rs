// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateRawAesKeyring`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::key_name) / [`set_key_name(Option<::std::string::String>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::set_key_name): (undocumented)<br>
    ///   - [`key_namespace(impl Into<Option<::std::string::String>>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::key_namespace) / [`set_key_namespace(Option<::std::string::String>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::set_key_namespace): (undocumented)<br>
    ///   - [`wrapping_alg(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg>>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::wrapping_alg) / [`set_wrapping_alg(Option<crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::set_wrapping_alg): (undocumented)<br>
    ///   - [`wrapping_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::wrapping_key) / [`set_wrapping_key(Option<::aws_smithy_types::Blob>)`](crate::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::set_wrapping_key): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_raw_aes_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_raw_aes_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateRawAesKeyringError>`](crate::operation::create_raw_aes_keyring::CreateRawAesKeyringError)
    pub fn create_raw_aes_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::builders::CreateRawAesKeyringFluentBuilder::new(self.clone())
    }
}
