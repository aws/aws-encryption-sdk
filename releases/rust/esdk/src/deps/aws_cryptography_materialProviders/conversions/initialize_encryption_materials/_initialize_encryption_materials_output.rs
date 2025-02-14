// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptionMaterials,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptionMaterials::EncryptionMaterials {
        algorithmSuite: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_info::to_dafny(&value.algorithm_suite.clone().unwrap())
,
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b),
    |v| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b),
)
,
 encryptedDataKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_keys.clone().unwrap(),
    |e| crate::deps::aws_cryptography_materialProviders::conversions::encrypted_data_key::to_dafny(&e.clone())
,
)
,
 requiredEncryptionContextKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.required_encryption_context_keys.clone().unwrap(),
    |e| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&e.as_bytes().to_vec(), |b| *b),
)
,
 plaintextDataKey: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext_data_key),
 signingKey: crate::standard_library_conversions::oblob_to_dafny(&value.signing_key),
 symmetricSigningKeys: ::dafny_runtime::Rc::new(match &value.symmetric_signing_keys {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::standard_library_conversions::blob_to_dafny(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptionMaterials,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials{
    crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::EncryptionMaterials::builder()
        .set_algorithm_suite(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_info::from_dafny(dafny_value.algorithmSuite().clone())
 ))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(k), |b| *b)).unwrap(),
    |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(v), |b| *b)).unwrap(),
)
 ))
 .set_encrypted_data_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedDataKeys(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey>| crate::deps::aws_cryptography_materialProviders::conversions::encrypted_data_key::from_dafny(e.clone())
,
)
 ))
 .set_required_encryption_context_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.requiredEncryptionContextKeys(),
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(e), |b| *b)).unwrap(),
)
 ))
 .set_plaintext_data_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.plaintextDataKey().clone()))
 .set_signing_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.signingKey().clone()))
 .set_symmetric_signing_keys(match (*dafny_value.symmetricSigningKeys()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| crate::standard_library_conversions::blob_from_dafny(e.clone()),
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
