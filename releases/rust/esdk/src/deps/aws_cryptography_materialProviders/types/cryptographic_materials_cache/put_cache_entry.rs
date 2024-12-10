// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
    /// Constructs a fluent builder for the [`PutCacheEntry`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bytes_used(impl Into<Option<::std::primitive::i32>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::bytes_used) / [`set_bytes_used(Option<::std::primitive::i32>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_bytes_used): (undocumented)<br>
    ///   - [`creation_time(impl Into<Option<::std::primitive::i64>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::creation_time) / [`set_creation_time(Option<::std::primitive::i64>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_creation_time): (undocumented)<br>
    ///   - [`expiry_time(impl Into<Option<::std::primitive::i64>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::expiry_time) / [`set_expiry_time(Option<::std::primitive::i64>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_expiry_time): (undocumented)<br>
    ///   - [`identifier(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::identifier) / [`set_identifier(Option<::aws_smithy_types::Blob>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_identifier): (undocumented)<br>
    ///   - [`materials(impl Into<Option<crate::deps::aws_cryptography_materialProviders::types::Materials>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::materials) / [`set_materials(Option<crate::deps::aws_cryptography_materialProviders::types::Materials>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_materials): (undocumented)<br>
    ///   - [`messages_used(impl Into<Option<::std::primitive::i32>>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::messages_used) / [`set_messages_used(Option<::std::primitive::i32>)`](crate::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::set_messages_used): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::put_cache_entry::Unit) with field(s):

    /// - On failure, responds with [`SdkError<PutCacheEntryError>`](crate::operation::put_cache_entry::PutCacheEntryError)
    pub fn put_cache_entry(&self) -> crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::builders::PutCacheEntryFluentBuilder::new(self.clone())
    }
}
