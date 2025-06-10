// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::AesCtr,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::AesCtr,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR::AES_CTR {
        keyLength: value.key_length.clone().unwrap(),
        nonceLength: value.nonce_length.clone().unwrap(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesCtr>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR,
        >,
    >,
> {
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR,
    >,
) -> crate::deps::aws_cryptography_primitives::types::AesCtr {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR,
) -> crate::deps::aws_cryptography_primitives::types::AesCtr {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR::AES_CTR {..} =>
            crate::deps::aws_cryptography_primitives::types::AesCtr::builder()
                .set_key_length(Some( dafny_value.keyLength() .clone() ))
 .set_nonce_length(Some( dafny_value.nonceLength() .clone() ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::_Wrappers_Compile::Option<
            ::dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_CTR,
            >,
        >,
    >,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesCtr> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
