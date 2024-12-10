// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Encrypt`](crate::operation::encrypt::builders::EncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`frame_length(impl Into<Option<::std::primitive::i64>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::frame_length) / [`set_frame_length(Option<::std::primitive::i64>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_frame_length): (undocumented)<br>
    ///   - [`keyring(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::keyring) / [`set_keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_keyring): (undocumented)<br>
    ///   - [`materials_manager(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::materials_manager) / [`set_materials_manager(Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_materials_manager): (undocumented)<br>
    ///   - [`plaintext(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::plaintext) / [`set_plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_plaintext): (undocumented)<br>
    /// - On success, responds with [`EncryptOutput`](crate::operation::encrypt::EncryptOutput) with field(s):
    ///   - [`algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>)`](crate::operation::encrypt::EncryptOutput::algorithm_suite_id): (undocumented)
    ///   - [`ciphertext(Option<::aws_smithy_types::Blob>)`](crate::operation::encrypt::EncryptOutput::ciphertext): (undocumented)
    ///   - [`encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::encrypt::EncryptOutput::encryption_context): (undocumented)
    /// - On failure, responds with [`SdkError<EncryptError>`](crate::operation::encrypt::EncryptError)
    pub fn encrypt(&self) -> crate::operation::encrypt::builders::EncryptFluentBuilder {
        crate::operation::encrypt::builders::EncryptFluentBuilder::new(self.clone())
    }
}
