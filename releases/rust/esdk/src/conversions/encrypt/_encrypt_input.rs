// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::encrypt::EncryptInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::EncryptInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::EncryptInput::EncryptInput {
        plaintext: crate::standard_library_conversions::blob_to_dafny(&value.plaintext.unwrap()),
 encryptionContext:
::std::rc::Rc::new(match &value.encryption_context {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b),
            |v| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 materialsManager: ::std::rc::Rc::new(match &value.materials_manager {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_manager::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 keyring: ::std::rc::Rc::new(match &value.keyring {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::keyring::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 algorithmSuiteId: ::std::rc::Rc::new(match &value.algorithm_suite_id {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::esdk_algorithm_suite_id::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 frameLength: crate::standard_library_conversions::olong_to_dafny(&value.frame_length),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::EncryptInput,
    >,
) -> crate::operation::encrypt::EncryptInput {
    crate::operation::encrypt::EncryptInput::builder()
        .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
 .set_encryption_context(match (*dafny_value.encryptionContext()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(k), |b| *b)).unwrap(),
                |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(v), |b| *b)).unwrap(),
            )
        ),
    _ => None
}
)
 .set_materials_manager(match (*dafny_value.materialsManager()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_manager::from_dafny(value.clone())),
    _ => None,
}
)
 .set_keyring(match (*dafny_value.keyring()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::keyring::from_dafny(value.clone())),
    _ => None,
}
)
 .set_algorithm_suite_id(match &**dafny_value.algorithmSuiteId() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::aws_cryptography_materialProviders::conversions::esdk_algorithm_suite_id::from_dafny(value)
    ),
    _ => None,
}
)
 .set_frame_length(crate::standard_library_conversions::olong_from_dafny(dafny_value.frameLength().clone()))
        .build()
        .unwrap()
}
