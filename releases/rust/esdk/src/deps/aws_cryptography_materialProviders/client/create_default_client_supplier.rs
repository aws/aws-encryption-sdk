// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::client::Client {
    /// Constructs a fluent builder for the [`CreateDefaultClientSupplier`](crate::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`CreateDefaultClientSupplierOutput`](crate::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput) with field(s):
    ///   - [`client(Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>)`](crate::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput::client): (undocumented)
    /// - On failure, responds with [`SdkError<CreateDefaultClientSupplierError>`](crate::operation::create_default_client_supplier::CreateDefaultClientSupplierError)
    pub fn create_default_client_supplier(&self) -> crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierFluentBuilder::new(self.clone())
    }
}
