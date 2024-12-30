// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
    /// Constructs a fluent builder for the [`UpdateUsageMetadata`](crate::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bytes_used(impl Into<Option<::std::primitive::i32>>)`](crate::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder::bytes_used) / [`set_bytes_used(Option<::std::primitive::i32>)`](crate::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder::set_bytes_used): (undocumented)<br>
    ///   - [`identifier(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder::identifier) / [`set_identifier(Option<::aws_smithy_types::Blob>)`](crate::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder::set_identifier): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::update_usage_metadata::Unit) with field(s):

    /// - On failure, responds with [`SdkError<UpdateUsageMetadataError>`](crate::operation::update_usage_metadata::UpdateUsageMetadataError)
    pub fn update_usage_metadata(&self) -> crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::builders::UpdateUsageMetadataFluentBuilder::new(self.clone())
    }
}
