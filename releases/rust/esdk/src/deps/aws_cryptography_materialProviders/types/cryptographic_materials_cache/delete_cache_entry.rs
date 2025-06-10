// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
    /// Constructs a fluent builder for the [`DeleteCacheEntry`](crate::operation::delete_cache_entry::builders::DeleteCacheEntryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::delete_cache_entry::builders::DeleteCacheEntryFluentBuilder::identifier) / [`set_identifier(Option<::aws_smithy_types::Blob>)`](crate::operation::delete_cache_entry::builders::DeleteCacheEntryFluentBuilder::set_identifier): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::delete_cache_entry::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DeleteCacheEntryError>`](crate::operation::delete_cache_entry::DeleteCacheEntryError)
    pub fn delete_cache_entry(&self) -> crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::builders::DeleteCacheEntryFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::builders::DeleteCacheEntryFluentBuilder::new(self.clone())
    }
}
