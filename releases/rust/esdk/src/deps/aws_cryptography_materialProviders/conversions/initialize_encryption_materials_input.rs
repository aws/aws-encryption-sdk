// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput::InitializeEncryptionMaterialsInput {
        algorithmSuiteId: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::to_dafny(&value.algorithm_suite_id.clone().unwrap())
,
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b),
    |v| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b),
)
,
 requiredEncryptionContextKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.required_encryption_context_keys.clone().unwrap(),
    |e| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&e.as_bytes().to_vec(), |b| *b),
)
,
 signingKey: crate::standard_library_conversions::oblob_to_dafny(&value.signing_key),
 verificationKey: crate::standard_library_conversions::oblob_to_dafny(&value.verification_key),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput,
) -> crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput::InitializeEncryptionMaterialsInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput::builder()
                .set_algorithm_suite_id(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithmSuiteId().clone())
 ))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(k), |b| *b)).unwrap(),
    |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(v), |b| *b)).unwrap(),
)
 ))
 .set_required_encryption_context_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.requiredEncryptionContextKeys(),
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(e), |b| *b)).unwrap(),
)
 ))
 .set_signing_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.signingKey().clone()))
 .set_verification_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.verificationKey().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeEncryptionMaterialsInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
