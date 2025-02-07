// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::CreateMultiKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput::CreateMultiKeyringInput {
        generator: ::dafny_runtime::Rc::new(match &value.generator {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::keyring::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 childKeyrings: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.child_keyrings.clone().unwrap(),
    |e| crate::deps::aws_cryptography_materialProviders::conversions::keyring::to_dafny(&e.clone())
,
)
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::CreateMultiKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::CreateMultiKeyringInput::builder()
        .set_generator(match (*dafny_value.generator()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::keyring::from_dafny(value.clone())),
    _ => None,
}
)
 .set_child_keyrings(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.childKeyrings(),
    |e: &::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::IKeyring>| crate::deps::aws_cryptography_materialProviders::conversions::keyring::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
