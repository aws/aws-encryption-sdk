// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput::GetCacheEntryInput {
        identifier: crate::standard_library_conversions::blob_to_dafny(&value.identifier.unwrap()),
 bytesUsed: crate::standard_library_conversions::olong_to_dafny(&value.bytes_used),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
) -> crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput::GetCacheEntryInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput::builder()
                .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
 .set_bytes_used(crate::standard_library_conversions::olong_from_dafny(dafny_value.bytesUsed().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput>
{
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
