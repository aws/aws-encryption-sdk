// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::HMacInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::HMacInput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput::HMacInput {
        digestAlgorithm: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 key: crate::standard_library_conversions::blob_to_dafny(&value.key.unwrap()),
 message: crate::standard_library_conversions::blob_to_dafny(&value.message.unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
    value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::HMacInput>,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Option<
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
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
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::HMacInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
) -> crate::deps::aws_cryptography_primitives::types::HMacInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput::HMacInput {..} =>
            crate::deps::aws_cryptography_primitives::types::HMacInput::builder()
                .set_digest_algorithm(Some( crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::HMacInput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
