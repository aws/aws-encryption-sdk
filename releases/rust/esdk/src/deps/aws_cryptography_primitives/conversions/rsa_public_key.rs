// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::RsaPublicKey,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::RsaPublicKey,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey::RSAPublicKey {
        lengthBits: value.length_bits.clone().unwrap(),
 pem: crate::standard_library_conversions::blob_to_dafny(&value.pem.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey,
    >,
) -> crate::deps::aws_cryptography_primitives::types::RsaPublicKey {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey,
) -> crate::deps::aws_cryptography_primitives::types::RsaPublicKey {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey::RSAPublicKey {..} =>
            crate::deps::aws_cryptography_primitives::types::RsaPublicKey::builder()
                .set_length_bits(Some( dafny_value.lengthBits() .clone() ))
 .set_pem(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.pem().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPublicKey,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
