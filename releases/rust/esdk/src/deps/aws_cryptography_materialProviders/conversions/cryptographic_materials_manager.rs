// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager,
>{
    let wrap = CryptographicMaterialsManagerWrapper { obj: value.clone() };
    let inner = ::dafny_runtime::Rc::new(::dafny_runtime::UnsafeCell::new(wrap));
    ::dafny_runtime::Object(Some(inner))
}

pub struct CryptographicMaterialsManagerWrapper {
  obj: crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
}

impl ::dafny_runtime::UpcastObject<::dafny_runtime::DynAny>
    for CryptographicMaterialsManagerWrapper
{
    ::dafny_runtime::UpcastObjectFn!(::dafny_runtime::DynAny);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef{
    let wrap = ICryptographicMaterialsManagerDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
      inner: ::dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ICryptographicMaterialsManagerDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager,
  >,
}

impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager
  for CryptographicMaterialsManagerWrapper
{
  fn r#_GetEncryptionMaterials_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetEncryptionMaterialsInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetEncryptionMaterialsOutput>,
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().get_encryption_materials(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}

fn r#_DecryptMaterials_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsOutput>,
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.lock().unwrap().decrypt_materials(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::dafny_runtime::Rc::new(result)
}
}

impl crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManager for ICryptographicMaterialsManagerDafnyWrapper
{
  fn get_encryption_materials(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsInput,
) -> Result<
  crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetEncryptionMaterials(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_output::from_dafny(inner_result.value().clone()),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}

fn decrypt_materials(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsInput,
) -> Result<
  crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput,
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).DecryptMaterials(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_output::from_dafny(inner_result.value().clone()),
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}
}
