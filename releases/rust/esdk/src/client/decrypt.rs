// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Decrypt`](crate::operation::decrypt::builders::DecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ciphertext(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::ciphertext) / [`set_ciphertext(Option<::aws_smithy_types::Blob>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_ciphertext): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`keyring(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::keyring) / [`set_keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_keyring): (undocumented)<br>
    ///   - [`materials_manager(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::materials_manager) / [`set_materials_manager(Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_materials_manager): (undocumented)<br>
    /// - On success, responds with [`DecryptOutput`](crate::operation::decrypt::DecryptOutput) with field(s):
    ///   - [`algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId>)`](crate::operation::decrypt::DecryptOutput::algorithm_suite_id): (undocumented)
    ///   - [`encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt::DecryptOutput::encryption_context): (undocumented)
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::decrypt::DecryptOutput::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<DecryptError>`](crate::operation::decrypt::DecryptError)
    pub fn decrypt(&self) -> crate::operation::decrypt::builders::DecryptFluentBuilder {
        crate::operation::decrypt::builders::DecryptFluentBuilder::new(self.clone())
    }
}
