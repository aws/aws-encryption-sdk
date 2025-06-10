// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRequiredEncryptionContextCMMInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRequiredEncryptionContextCMMInput::CreateRequiredEncryptionContextCMMInput {
        underlyingCMM: ::dafny_runtime::Rc::new(match &value.underlying_cmm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_manager::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 keyring: ::dafny_runtime::Rc::new(match &value.keyring {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::keyring::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRequiredEncryptionContextCMMInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput::builder()
        .set_underlying_cmm(match (*dafny_value.underlyingCMM()).as_ref() {
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
 .set_required_encryption_context_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.requiredEncryptionContextKeys(),
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&::std::borrow::Borrow::borrow(e), |b| *b)).unwrap(),
)
 ))
        .build()
        .unwrap()
}
