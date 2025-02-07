// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::AesEncryptOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::AesEncryptOutput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput::AESEncryptOutput {
        cipherText: crate::standard_library_conversions::blob_to_dafny(&value.cipher_text.unwrap()),
 authTag: crate::standard_library_conversions::blob_to_dafny(&value.auth_tag.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesEncryptOutput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::AesEncryptOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
) -> crate::deps::aws_cryptography_primitives::types::AesEncryptOutput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput::AESEncryptOutput {..} =>
            crate::deps::aws_cryptography_primitives::types::AesEncryptOutput::builder()
                .set_cipher_text(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.cipherText().clone())))
 .set_auth_tag(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.authTag().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesEncryptOutput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
