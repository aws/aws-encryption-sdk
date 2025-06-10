// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput::CreateRawRsaKeyringInput {
        keyNamespace: crate::standard_library_conversions::ostring_to_dafny(&value.key_namespace) .Extract(),
 keyName: crate::standard_library_conversions::ostring_to_dafny(&value.key_name) .Extract(),
 paddingScheme: crate::deps::aws_cryptography_materialProviders::conversions::padding_scheme::to_dafny(value.padding_scheme.clone().unwrap()),
 publicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 privateKey: crate::standard_library_conversions::oblob_to_dafny(&value.private_key),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateRawRsaKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput::builder()
        .set_key_namespace(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyNamespace()) ))
 .set_key_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyName()) ))
 .set_padding_scheme(Some( crate::deps::aws_cryptography_materialProviders::conversions::padding_scheme::from_dafny(dafny_value.paddingScheme()) ))
 .set_public_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.publicKey().clone()))
 .set_private_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.privateKey().clone()))
        .build()
        .unwrap()
}
