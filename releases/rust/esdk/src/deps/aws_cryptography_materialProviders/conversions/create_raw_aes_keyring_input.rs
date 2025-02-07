// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput::CreateRawAesKeyringInput {
        keyNamespace: crate::standard_library_conversions::ostring_to_dafny(&value.key_namespace) .Extract(),
 keyName: crate::standard_library_conversions::ostring_to_dafny(&value.key_name) .Extract(),
 wrappingKey: crate::standard_library_conversions::blob_to_dafny(&value.wrapping_key.unwrap()),
 wrappingAlg: crate::deps::aws_cryptography_materialProviders::conversions::aes_wrapping_alg::to_dafny(value.wrapping_alg.clone().unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput::CreateRawAesKeyringInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput::builder()
                .set_key_namespace(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyNamespace()) ))
 .set_key_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyName()) ))
 .set_wrapping_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.wrappingKey().clone())))
 .set_wrapping_alg(Some( crate::deps::aws_cryptography_materialProviders::conversions::aes_wrapping_alg::from_dafny(dafny_value.wrappingAlg()) ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
