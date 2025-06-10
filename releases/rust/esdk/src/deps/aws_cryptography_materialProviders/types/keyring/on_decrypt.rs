// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef {
    /// Constructs a fluent builder for the [`OnDecrypt`](crate::operation::on_decrypt::builders::OnDecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encrypted_data_keys(impl Into<Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>>>)`](crate::operation::on_decrypt::builders::OnDecryptFluentBuilder::encrypted_data_keys) / [`set_encrypted_data_keys(Option<::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>>)`](crate::operation::on_decrypt::builders::OnDecryptFluentBuilder::set_encrypted_data_keys): (undocumented)<br>
    ///   - [`materials(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials>>)`](crate::operation::on_decrypt::builders::OnDecryptFluentBuilder::materials) / [`set_materials(Option<crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials>)`](crate::operation::on_decrypt::builders::OnDecryptFluentBuilder::set_materials): (undocumented)<br>
    /// - On success, responds with [`OnDecryptOutput`](crate::operation::on_decrypt::OnDecryptOutput) with field(s):
    ///   - [`materials(Option<crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials>)`](crate::operation::on_decrypt::OnDecryptOutput::materials): (undocumented)
    /// - On failure, responds with [`SdkError<OnDecryptError>`](crate::operation::on_decrypt::OnDecryptError)
    pub fn on_decrypt(&self) -> crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::builders::OnDecryptFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::builders::OnDecryptFluentBuilder::new(self.clone())
    }
}
