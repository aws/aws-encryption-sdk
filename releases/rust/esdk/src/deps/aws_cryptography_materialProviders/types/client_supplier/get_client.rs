// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef {
    /// Constructs a fluent builder for the [`GetClient`](crate::operation::get_client::builders::GetClientFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`region(impl Into<Option<::std::string::String>>)`](crate::operation::get_client::builders::GetClientFluentBuilder::region) / [`set_region(Option<::std::string::String>)`](crate::operation::get_client::builders::GetClientFluentBuilder::set_region): (undocumented)<br>
    /// - On success, responds with [`GetClientOutput`](crate::operation::get_client::GetClientOutput) with field(s):
    ///   - [`client(Option<crate::deps::com_amazonaws_kms::client::Client>)`](crate::operation::get_client::GetClientOutput::client): (undocumented)
    /// - On failure, responds with [`SdkError<GetClientError>`](crate::operation::get_client::GetClientError)
    pub fn get_client(&self) -> crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientFluentBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientFluentBuilder::new(self.clone())
    }
}
