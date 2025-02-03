// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::dafny_runtime::Object<
    dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IKeyring,
> {
    let wrap = KeyringWrapper { obj: value.clone() };
    let inner = ::dafny_runtime::Rc::new(::dafny_runtime::UnsafeCell::new(wrap));
    ::dafny_runtime::Object(Some(inner))
}

pub struct KeyringWrapper {
    obj: crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
}

impl ::dafny_runtime::UpcastObject<::dafny_runtime::DynAny> for KeyringWrapper {
    ::dafny_runtime::UpcastObjectFn!(::dafny_runtime::DynAny);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IKeyring,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef {
    let wrap = IKeyringDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef {
        inner: ::dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(wrap)),
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IKeyringDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IKeyring,
  >,
}

impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::IKeyring
    for KeyringWrapper
{
    fn r#_OnEncrypt_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::OnEncryptInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::OnEncryptOutput>,
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::on_encrypt::_on_encrypt_input::from_dafny(input.clone());
        let inner_result = self.obj.inner.lock().unwrap().on_encrypt(inner_input);
        let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::on_encrypt::_on_encrypt_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
        ::dafny_runtime::Rc::new(result)
    }

fn r#_OnDecrypt_k(
    &self,
    input: &::dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::OnDecryptInput>,
) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::OnDecryptOutput>,
        ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Error>,
    >,
>
{
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::on_decrypt::_on_decrypt_input::from_dafny(input.clone());
        let inner_result = self.obj.inner.lock().unwrap().on_decrypt(inner_input);
        let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::deps::aws_cryptography_materialProviders::conversions::on_decrypt::_on_decrypt_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::deps::aws_cryptography_materialProviders::conversions::error::to_dafny(x),
        },
    };
        ::dafny_runtime::Rc::new(result)
    }
}

impl crate::deps::aws_cryptography_materialProviders::types::keyring::Keyring
    for IKeyringDafnyWrapper
{
    fn on_encrypt(
        &self,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput,
    ) -> Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::on_encrypt::_on_encrypt_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(self.obj.clone()).OnEncrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
          crate::deps::aws_cryptography_materialProviders::conversions::on_encrypt::_on_encrypt_output::from_dafny(inner_result.value().clone()),
      )
        } else {
            Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }

    fn on_decrypt(
        &self,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput,
    ) -> Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let inner_input = crate::deps::aws_cryptography_materialProviders::conversions::on_decrypt::_on_decrypt_input::to_dafny(input);
        let inner_result = ::dafny_runtime::md!(self.obj.clone()).OnDecrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
          crate::deps::aws_cryptography_materialProviders::conversions::on_decrypt::_on_decrypt_output::from_dafny(inner_result.value().clone()),
      )
        } else {
            Err(
                crate::deps::aws_cryptography_materialProviders::conversions::error::from_dafny(
                    inner_result.error().clone(),
                ),
            )
        }
    }
}
