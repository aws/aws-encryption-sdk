// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::AesDecryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::AesDecryptInput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput::AESDecryptInput {
        encAlg: crate::deps::aws_cryptography_primitives::conversions::aes_gcm::to_dafny(&value.enc_alg.clone().unwrap())
,
 key: crate::standard_library_conversions::blob_to_dafny(&value.key.unwrap()),
 cipherTxt: crate::standard_library_conversions::blob_to_dafny(&value.cipher_txt.unwrap()),
 authTag: crate::standard_library_conversions::blob_to_dafny(&value.auth_tag.unwrap()),
 iv: crate::standard_library_conversions::blob_to_dafny(&value.iv.unwrap()),
 aad: crate::standard_library_conversions::blob_to_dafny(&value.aad.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesDecryptInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::AesDecryptInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
) -> crate::deps::aws_cryptography_primitives::types::AesDecryptInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput::AESDecryptInput {..} =>
            crate::deps::aws_cryptography_primitives::types::AesDecryptInput::builder()
                .set_enc_alg(Some( crate::deps::aws_cryptography_primitives::conversions::aes_gcm::from_dafny(dafny_value.encAlg().clone())
 ))
 .set_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_cipher_txt(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.cipherTxt().clone())))
 .set_auth_tag(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.authTag().clone())))
 .set_iv(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.iv().clone())))
 .set_aad(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.aad().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesDecryptInput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
