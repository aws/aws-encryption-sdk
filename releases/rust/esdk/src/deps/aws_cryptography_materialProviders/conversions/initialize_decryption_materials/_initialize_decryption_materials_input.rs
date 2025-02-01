// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::InitializeDecryptionMaterialsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeDecryptionMaterialsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeDecryptionMaterialsInput::InitializeDecryptionMaterialsInput {
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
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::InitializeDecryptionMaterialsInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::InitializeDecryptionMaterialsInput{
    crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::InitializeDecryptionMaterialsInput::builder()
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
        .build()
        .unwrap()
}
