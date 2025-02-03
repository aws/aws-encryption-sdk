// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::CreateRawAesKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput::CreateRawAesKeyringInput {
        keyNamespace: crate::standard_library_conversions::ostring_to_dafny(&value.key_namespace) .Extract(),
 keyName: crate::standard_library_conversions::ostring_to_dafny(&value.key_name) .Extract(),
 wrappingKey: crate::standard_library_conversions::blob_to_dafny(&value.wrapping_key.unwrap()),
 wrappingAlg: crate::deps::aws_cryptography_materialProviders::conversions::aes_wrapping_alg::to_dafny(value.wrapping_alg.clone().unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawAesKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::CreateRawAesKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::CreateRawAesKeyringInput::builder()
        .set_key_namespace(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyNamespace()) ))
 .set_key_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyName()) ))
 .set_wrapping_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.wrappingKey().clone())))
 .set_wrapping_alg(Some( crate::deps::aws_cryptography_materialProviders::conversions::aes_wrapping_alg::from_dafny(dafny_value.wrappingAlg()) ))
        .build()
        .unwrap()
}
