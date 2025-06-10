// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateMultiKeyring`](crate::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`child_keyrings(impl Into<Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>>)`](crate::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder::child_keyrings) / [`set_child_keyrings(Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>)`](crate::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder::set_child_keyrings): (undocumented)<br>
    ///   - [`generator(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>)`](crate::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder::generator) / [`set_generator(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder::set_generator): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_multi_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_multi_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateMultiKeyringError>`](crate::operation::create_multi_keyring::CreateMultiKeyringError)
    pub fn create_multi_keyring(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::builders::CreateMultiKeyringFluentBuilder::new(self.clone())
    }
}
