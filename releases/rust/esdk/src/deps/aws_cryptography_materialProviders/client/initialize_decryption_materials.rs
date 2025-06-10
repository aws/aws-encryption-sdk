// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`InitializeDecryptionMaterials`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`required_encryption_context_keys(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::required_encryption_context_keys) / [`set_required_encryption_context_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::set_required_encryption_context_keys): (undocumented)<br>
    /// - On success, responds with [`DecryptionMaterials`](crate::operation::initialize_decryption_materials::DecryptionMaterials) with field(s):
    ///   - [`algorithm_suite(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::algorithm_suite): (undocumented)
    ///   - [`encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::encryption_context): (undocumented)
    ///   - [`plaintext_data_key(Option<::aws_smithy_types::Blob>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::plaintext_data_key): (undocumented)
    ///   - [`required_encryption_context_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::required_encryption_context_keys): (undocumented)
    ///   - [`symmetric_signing_key(Option<::aws_smithy_types::Blob>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::symmetric_signing_key): (undocumented)
    ///   - [`verification_key(Option<::aws_smithy_types::Blob>)`](crate::operation::initialize_decryption_materials::DecryptionMaterials::verification_key): (undocumented)
    /// - On failure, responds with [`SdkError<InitializeDecryptionMaterialsError>`](crate::operation::initialize_decryption_materials::InitializeDecryptionMaterialsError)
    pub fn initialize_decryption_materials(&self) -> crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::builders::InitializeDecryptionMaterialsFluentBuilder::new(self.clone())
    }
}
