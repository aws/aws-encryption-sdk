// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`ValidEncryptionMaterialsTransition`](crate::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`start(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials>>)`](crate::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder::start) / [`set_start(Option<crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials>)`](crate::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder::set_start): (undocumented)<br>
    ///   - [`stop(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials>>)`](crate::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder::stop) / [`set_stop(Option<crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials>)`](crate::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder::set_stop): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::valid_encryption_materials_transition::Unit) with field(s):

    /// - On failure, responds with [`SdkError<ValidEncryptionMaterialsTransitionError>`](crate::operation::valid_encryption_materials_transition::ValidEncryptionMaterialsTransitionError)
    pub fn valid_encryption_materials_transition(&self) -> crate::deps::aws_cryptography_materialProviders::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::valid_encryption_materials_transition::builders::ValidEncryptionMaterialsTransitionFluentBuilder::new(self.clone())
    }
}
