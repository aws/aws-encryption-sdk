// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey::EncryptedDataKey {
        keyProviderId: dafny_runtime::Rc::new(match value.key_provider_id {
  Some(s) => crate::_Wrappers_Compile::Option::Some { value: dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&s.as_bytes().to_vec(), |b| *b) },
  None => crate::_Wrappers_Compile::Option::None {},
}).Extract(),
 keyProviderInfo: crate::standard_library_conversions::blob_to_dafny(&value.key_provider_info.unwrap()),
 ciphertext: crate::standard_library_conversions::blob_to_dafny(&value.ciphertext.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey,
>>>{
    ::dafny_runtime::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::dafny_runtime::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey,
) -> crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey::EncryptedDataKey {..} =>
            crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey::builder()
                .set_key_provider_id(Some(::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(dafny_value.keyProviderId()), |b| *b)).unwrap()))
 .set_key_provider_info(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.keyProviderInfo().clone())))
 .set_ciphertext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ciphertext().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>
{
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
