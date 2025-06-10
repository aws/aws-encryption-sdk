// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput::GetCacheEntryOutput {
        materials: crate::deps::aws_cryptography_materialProviders::conversions::materials::to_dafny(&value.materials.clone().unwrap())
,
 creationTime: value.creation_time.clone().unwrap(),
 expiryTime: value.expiry_time.clone().unwrap(),
 messagesUsed: value.messages_used.clone().unwrap(),
 bytesUsed: value.bytes_used.clone().unwrap(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
) -> crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput::GetCacheEntryOutput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput::builder()
                .set_materials(Some( crate::deps::aws_cryptography_materialProviders::conversions::materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_creation_time(Some( dafny_value.creationTime() .clone() ))
 .set_expiry_time(Some( dafny_value.expiryTime() .clone() ))
 .set_messages_used(Some( dafny_value.messagesUsed() .clone() ))
 .set_bytes_used(Some( dafny_value.bytesUsed() .clone() ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
