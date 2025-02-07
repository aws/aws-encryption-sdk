// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput::CreateCryptographicMaterialsCacheInput {
        cache: crate::deps::aws_cryptography_materialProviders::conversions::cache_type::to_dafny(&value.cache.clone().unwrap())
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput
{
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput
{
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput::CreateCryptographicMaterialsCacheInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput::builder()
                .set_cache(Some( crate::deps::aws_cryptography_materialProviders::conversions::cache_type::from_dafny(dafny_value.cache().clone())
 ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
