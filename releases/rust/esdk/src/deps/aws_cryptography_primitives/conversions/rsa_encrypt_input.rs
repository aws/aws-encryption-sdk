// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::RsaEncryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::RsaEncryptInput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput::RSAEncryptInput {
        padding: crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 publicKey: crate::standard_library_conversions::blob_to_dafny(&value.public_key.unwrap()),
 plaintext: crate::standard_library_conversions::blob_to_dafny(&value.plaintext.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaEncryptInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::RsaEncryptInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
) -> crate::deps::aws_cryptography_primitives::types::RsaEncryptInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput::RSAEncryptInput {..} =>
            crate::deps::aws_cryptography_primitives::types::RsaEncryptInput::builder()
                .set_padding(Some( crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::from_dafny(dafny_value.padding()) ))
 .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
 .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaEncryptInput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
