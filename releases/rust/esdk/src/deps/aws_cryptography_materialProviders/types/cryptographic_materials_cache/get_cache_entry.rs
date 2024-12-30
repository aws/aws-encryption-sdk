// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
    /// Constructs a fluent builder for the [`GetCacheEntry`](crate::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bytes_used(impl Into<Option<::std::primitive::i64>>)`](crate::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder::bytes_used) / [`set_bytes_used(Option<::std::primitive::i64>)`](crate::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder::set_bytes_used): (undocumented)<br>
    ///   - [`identifier(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder::identifier) / [`set_identifier(Option<::aws_smithy_types::Blob>)`](crate::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder::set_identifier): (undocumented)<br>
    /// - On success, responds with [`GetCacheEntryOutput`](crate::operation::get_cache_entry::GetCacheEntryOutput) with field(s):
    ///   - [`bytes_used(Option<::std::primitive::i32>)`](crate::operation::get_cache_entry::GetCacheEntryOutput::bytes_used): (undocumented)
    ///   - [`creation_time(Option<::std::primitive::i64>)`](crate::operation::get_cache_entry::GetCacheEntryOutput::creation_time): (undocumented)
    ///   - [`expiry_time(Option<::std::primitive::i64>)`](crate::operation::get_cache_entry::GetCacheEntryOutput::expiry_time): (undocumented)
    ///   - [`materials(Option<crate::deps::aws_cryptography_materialProviders::types::Materials>)`](crate::operation::get_cache_entry::GetCacheEntryOutput::materials): (undocumented)
    ///   - [`messages_used(Option<::std::primitive::i32>)`](crate::operation::get_cache_entry::GetCacheEntryOutput::messages_used): (undocumented)
    /// - On failure, responds with [`SdkError<GetCacheEntryError>`](crate::operation::get_cache_entry::GetCacheEntryError)
    pub fn get_cache_entry(&self) -> crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::builders::GetCacheEntryFluentBuilder::new(self.clone())
    }
}
