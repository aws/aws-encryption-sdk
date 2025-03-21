// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::AesGcm,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::AesGcm,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM::AES_GCM {
        keyLength: value.key_length.clone().unwrap(),
        tagLength: value.tag_length.clone().unwrap(),
        ivLength: value.iv_length.clone().unwrap(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM,
    >,
) -> crate::deps::aws_cryptography_primitives::types::AesGcm {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM,
) -> crate::deps::aws_cryptography_primitives::types::AesGcm {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM::AES_GCM {..} =>
            crate::deps::aws_cryptography_primitives::types::AesGcm::builder()
                .set_key_length(Some( dafny_value.keyLength() .clone() ))
 .set_tag_length(Some( dafny_value.tagLength() .clone() ))
 .set_iv_length(Some( dafny_value.ivLength() .clone() ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::_Wrappers_Compile::Option<
            ::dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::primitives::internaldafny::types::AES_GCM,
            >,
        >,
    >,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
