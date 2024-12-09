// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait CryptographicMaterialsCache {
    fn put_cache_entry(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput,
  ) -> Result<
    (),
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;

  fn update_usage_metadata(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::UpdateUsageMetadataInput,
  ) -> Result<
    (),
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;

  fn get_cache_entry(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput,
  ) -> Result<
    crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput,
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;

  fn delete_cache_entry(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput,
  ) -> Result<
    (),
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
/// A reference to a CryptographicMaterialsCache
pub struct CryptographicMaterialsCacheRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn CryptographicMaterialsCache>>
}

impl<T : CryptographicMaterialsCache + 'static> From<T> for CryptographicMaterialsCacheRef {
    fn from(value: T) -> Self {
        Self { inner: std::rc::Rc::new(std::cell::RefCell::new(value)) }
    }
}

impl ::std::cmp::PartialEq for CryptographicMaterialsCacheRef {
    fn eq(&self, other: &CryptographicMaterialsCacheRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for CryptographicMaterialsCacheRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CryptographicMaterialsCacheRef>")
    }
}

mod put_cache_entry;

mod update_usage_metadata;

mod get_cache_entry;

mod delete_cache_entry;
