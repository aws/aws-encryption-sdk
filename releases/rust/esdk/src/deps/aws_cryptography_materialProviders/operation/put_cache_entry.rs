// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `PutCacheEntry`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutCacheEntry;
impl PutCacheEntry {
    /// Creates a new `PutCacheEntry`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        cryptographic_materials_cache: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput,
    ) -> ::std::result::Result<
        (),
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        crate::deps::aws_cryptography_materialProviders::validation::validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_for_CryptographicMaterialsCache_PutCacheEntry(&input)
            .map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err)?;
        cryptographic_materials_cache
            .inner
            .lock()
            .unwrap()
            .put_cache_entry(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::_unit::Unit;

pub use crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::_put_cache_entry_input::PutCacheEntryInput;

pub(crate) mod _unit;

pub(crate) mod _put_cache_entry_input;

/// Builders
pub mod builders;
