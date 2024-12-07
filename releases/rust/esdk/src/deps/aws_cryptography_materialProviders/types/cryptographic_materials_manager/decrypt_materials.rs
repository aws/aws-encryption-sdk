// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
    /// Constructs a fluent builder for the [`DecryptMaterials`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`commitment_policy(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::commitment_policy) / [`set_commitment_policy(Option<crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::set_commitment_policy): (undocumented)<br>
    ///   - [`encrypted_data_keys(impl Into<Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::encrypted_data_keys) / [`set_encrypted_data_keys(Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::set_encrypted_data_keys): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`reproduced_encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::reproduced_encryption_context) / [`set_reproduced_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::set_reproduced_encryption_context): (undocumented)<br>
    /// - On success, responds with [`DecryptMaterialsOutput`](crate::operation::decrypt_materials::DecryptMaterialsOutput) with field(s):
    ///   - [`decryption_materials(Option<crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials>)`](crate::operation::decrypt_materials::DecryptMaterialsOutput::decryption_materials): (undocumented)
    /// - On failure, responds with [`SdkError<DecryptMaterialsError>`](crate::operation::decrypt_materials::DecryptMaterialsError)
    pub fn decrypt_materials(&self) -> crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::builders::DecryptMaterialsFluentBuilder::new(self.clone())
    }
}
