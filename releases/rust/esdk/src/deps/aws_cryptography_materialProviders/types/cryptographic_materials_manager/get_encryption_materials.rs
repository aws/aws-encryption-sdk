// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
    /// Constructs a fluent builder for the [`GetEncryptionMaterials`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`commitment_policy(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::commitment_policy) / [`set_commitment_policy(Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::set_commitment_policy): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`max_plaintext_length(impl Into<Option<::std::primitive::i64>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::max_plaintext_length) / [`set_max_plaintext_length(Option<::std::primitive::i64>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::set_max_plaintext_length): (undocumented)<br>
    ///   - [`required_encryption_context_keys(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::required_encryption_context_keys) / [`set_required_encryption_context_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::set_required_encryption_context_keys): (undocumented)<br>
    /// - On success, responds with [`GetEncryptionMaterialsOutput`](crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput) with field(s):
    ///   - [`encryption_materials(Option<crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials>)`](crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput::encryption_materials): (undocumented)
    /// - On failure, responds with [`SdkError<GetEncryptionMaterialsError>`](crate::operation::get_encryption_materials::GetEncryptionMaterialsError)
    pub fn get_encryption_materials(&self) -> crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::builders::GetEncryptionMaterialsFluentBuilder::new(self.clone())
    }
}
