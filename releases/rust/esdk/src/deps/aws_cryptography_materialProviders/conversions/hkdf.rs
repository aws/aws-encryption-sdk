// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::Hkdf,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::Hkdf,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF {
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF::HKDF {
        hmac: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(
            value.hmac.clone().unwrap(),
        ),
        saltLength: value.salt_length.clone().unwrap(),
        inputKeyLength: value.input_key_length.clone().unwrap(),
        outputKeyLength: value.output_key_length.clone().unwrap(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Hkdf>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF,
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::Hkdf {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF,
) -> crate::deps::aws_cryptography_materialProviders::types::Hkdf {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF::HKDF {..} =>
            crate::deps::aws_cryptography_materialProviders::types::Hkdf::builder()
                .set_hmac(Some( crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::from_dafny(dafny_value.hmac()) ))
 .set_salt_length(Some( dafny_value.saltLength() .clone() ))
 .set_input_key_length(Some( dafny_value.inputKeyLength() .clone() ))
 .set_output_key_length(Some( dafny_value.outputKeyLength() .clone() ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::HKDF,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Hkdf> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
