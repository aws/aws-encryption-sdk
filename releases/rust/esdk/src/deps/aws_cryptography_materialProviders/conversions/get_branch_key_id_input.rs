// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput::GetBranchKeyIdInput {
        encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b),
    |v| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b),
)
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput,
) -> crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput::GetBranchKeyIdInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput::builder()
                .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(k), |b| *b)).unwrap(),
    |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(v), |b| *b)).unwrap(),
)
 ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
