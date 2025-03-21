// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput::CreateRawRsaKeyringInput {
        keyNamespace: crate::standard_library_conversions::ostring_to_dafny(&value.key_namespace) .Extract(),
 keyName: crate::standard_library_conversions::ostring_to_dafny(&value.key_name) .Extract(),
 paddingScheme: crate::deps::aws_cryptography_materialProviders::conversions::padding_scheme::to_dafny(value.padding_scheme.clone().unwrap()),
 publicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 privateKey: crate::standard_library_conversions::oblob_to_dafny(&value.private_key),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput::CreateRawRsaKeyringInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput::builder()
                .set_key_namespace(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyNamespace()) ))
 .set_key_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyName()) ))
 .set_padding_scheme(Some( crate::deps::aws_cryptography_materialProviders::conversions::padding_scheme::from_dafny(dafny_value.paddingScheme()) ))
 .set_public_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.publicKey().clone()))
 .set_private_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.privateKey().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
