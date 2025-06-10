// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache,
>{
    let wrap = CryptographicMaterialsCacheWrapper { obj: value.clone() };
    let inner = ::dafny_runtime::Rc::new(::dafny_runtime::UnsafeCell::new(wrap));
    ::dafny_runtime::Object(Some(inner))
}

pub struct CryptographicMaterialsCacheWrapper {
  obj: crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
}

impl ::dafny_runtime::UpcastObject<::dafny_runtime::DynAny> for CryptographicMaterialsCacheWrapper {
    ::dafny_runtime::UpcastObjectFn!(::dafny_runtime::DynAny);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef{
    let wrap = ICryptographicMaterialsCacheDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef {
      inner: ::dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ICryptographicMaterialsCacheDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache,
  >,
}

impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache
  for CryptographicMaterialsCacheWrapper
{
  fn r#_PutCacheEntry_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        (),
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::put_cache_entry::_put_cache_entry_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().put_cache_entry(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: (),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}

fn r#_UpdateUsageMetadata_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::UpdateUsageMetadataInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        (),
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::update_usage_metadata::_update_usage_metadata_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().update_usage_metadata(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: (),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}

fn r#_GetCacheEntry_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput>,
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_cache_entry::_get_cache_entry_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().get_cache_entry(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::get_cache_entry::_get_cache_entry_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}

fn r#_DeleteCacheEntry_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        (),
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::delete_cache_entry::_delete_cache_entry_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().delete_cache_entry(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: (),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}
}

impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCache for ICryptographicMaterialsCacheDafnyWrapper
{
  fn put_cache_entry(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput,
) -> Result<
  (),
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::put_cache_entry::_put_cache_entry_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).PutCacheEntry(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          (),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}

fn update_usage_metadata(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::UpdateUsageMetadataInput,
) -> Result<
  (),
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::update_usage_metadata::_update_usage_metadata_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).UpdateUsageMetadata(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          (),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}

fn get_cache_entry(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput,
) -> Result<
  crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput,
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_cache_entry::_get_cache_entry_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetCacheEntry(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          crate::deps::aws_cryptography_materialProviders::conversions::get_cache_entry::_get_cache_entry_output::from_dafny(inner_result.value().clone()),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}

fn delete_cache_entry(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput,
) -> Result<
  (),
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::delete_cache_entry::_delete_cache_entry_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).DeleteCacheEntry(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          (),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}
}
