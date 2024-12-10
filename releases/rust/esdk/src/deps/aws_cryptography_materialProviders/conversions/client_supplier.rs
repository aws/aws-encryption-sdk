// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IClientSupplier,
> {
  let wrap = ClientSupplierWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct ClientSupplierWrapper {
  obj: crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for ClientSupplierWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IClientSupplier,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef {
    let wrap = IClientSupplierDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IClientSupplierDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IClientSupplier,
  >,
}

impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::IClientSupplier
  for ClientSupplierWrapper
{
  fn r#_GetClient_k(
    &self,
    input: &::std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetClientInput>,
) -> ::std::rc::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>,
        ::std::rc::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_client::_get_client_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.borrow_mut().get_client(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::com_amazonaws_kms::conversions::client::to_dafny(&x.clone())
,
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
    ::std::rc::Rc::new(result)
}
}

impl crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplier for IClientSupplierDafnyWrapper
{
  fn get_client(
  &self,
  input: crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput,
) -> Result<
  crate::deps::com_amazonaws_kms::client::Client,
  crate::deps::aws_cryptography_materialProviders::types::error::Error,
> {
  let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::get_client::_get_client_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).GetClient(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          crate::deps::com_amazonaws_kms::conversions::client::from_dafny(inner_result.value().clone())
,
      )
  } else {
      Err(crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}
}
