// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateDefaultCryptographicMaterialsManager`](crate::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyring(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>>)`](crate::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerFluentBuilder::keyring) / [`set_keyring(Option<crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef>)`](crate::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerFluentBuilder::set_keyring): (undocumented)<br>
    /// - On success, responds with [`CreateCryptographicMaterialsManagerOutput`](crate::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput) with field(s):
    ///   - [`materials_manager(Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput::materials_manager): (undocumented)
    /// - On failure, responds with [`SdkError<CreateDefaultCryptographicMaterialsManagerError>`](crate::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerError)
    pub fn create_default_cryptographic_materials_manager(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::builders::CreateDefaultCryptographicMaterialsManagerFluentBuilder::new(self.clone())
    }
}
