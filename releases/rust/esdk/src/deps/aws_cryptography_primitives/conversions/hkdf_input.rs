// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::HkdfInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::HkdfInput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput::HkdfInput {
        digestAlgorithm: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 salt: crate::standard_library_conversions::oblob_to_dafny(&value.salt),
 ikm: crate::standard_library_conversions::blob_to_dafny(&value.ikm.unwrap()),
 info: crate::standard_library_conversions::blob_to_dafny(&value.info.unwrap()),
 expectedLength: value.expected_length.clone().unwrap(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::HkdfInput>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::HkdfInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
) -> crate::deps::aws_cryptography_primitives::types::HkdfInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput::HkdfInput {..} =>
            crate::deps::aws_cryptography_primitives::types::HkdfInput::builder()
                .set_digest_algorithm(Some( crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_salt(crate::standard_library_conversions::oblob_from_dafny(dafny_value.salt().clone()))
 .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_info(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.info().clone())))
 .set_expected_length(Some( dafny_value.expectedLength() .clone() ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::HkdfInput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
