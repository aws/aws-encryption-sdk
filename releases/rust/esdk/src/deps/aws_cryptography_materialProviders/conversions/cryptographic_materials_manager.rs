// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager,
> {
  let wrap = CryptographicMaterialsManagerWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct CryptographicMaterialsManagerWrapper {
  obj: crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for CryptographicMaterialsManagerWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsManager,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
    let wrap = ICryptographicMaterialsManagerDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
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
    input: &::std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetEncryptionMaterialsInput>,
) -> ::std::rc::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::std::rc::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetEncryptionMaterialsOutput>,
        ::std::rc::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.borrow_mut().get_encryption_materials(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::get_encryption_materials::_get_encryption_materials_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::std::rc::Rc::new(result)
}

fn r#_DecryptMaterials_k(
    &self,
    input: &::std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsInput>,
) -> ::std::rc::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::std::rc::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsOutput>,
        ::std::rc::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.borrow_mut().decrypt_materials(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::decrypt_materials::_decrypt_materials_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::std::rc::Rc::new(result)
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
