// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials::DecryptionMaterials {
        algorithmSuite: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_info::to_dafny(&value.algorithm_suite.clone().unwrap())
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
 plaintextDataKey: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext_data_key),
 verificationKey: crate::standard_library_conversions::oblob_to_dafny(&value.verification_key),
 symmetricSigningKey: crate::standard_library_conversions::oblob_to_dafny(&value.symmetric_signing_key),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials,
) -> crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials::DecryptionMaterials {..} =>
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials::builder()
                .set_algorithm_suite(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_info::from_dafny(dafny_value.algorithmSuite().clone())
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
 .set_plaintext_data_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.plaintextDataKey().clone()))
 .set_verification_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.verificationKey().clone()))
 .set_symmetric_signing_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.symmetricSigningKey().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptionMaterials,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
