// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetCacheEntry`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetCacheEntry;
impl GetCacheEntry {
    /// Creates a new `GetCacheEntry`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        cryptographic_materials_cache: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_for_CryptographicMaterialsCache_GetCacheEntry(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        cryptographic_materials_cache
            .inner
            .lock()
            .unwrap()
            .get_cache_entry(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::_get_cache_entry_output::GetCacheEntryOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::_get_cache_entry_input::GetCacheEntryInput;

pub(crate) mod _get_cache_entry_output;

pub(crate) mod _get_cache_entry_input;

/// Builders
pub mod builders;
