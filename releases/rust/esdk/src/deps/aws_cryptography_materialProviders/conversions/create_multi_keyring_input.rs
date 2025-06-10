// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput::CreateMultiKeyringInput {
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
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput::CreateMultiKeyringInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput::builder()
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
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateMultiKeyringInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
