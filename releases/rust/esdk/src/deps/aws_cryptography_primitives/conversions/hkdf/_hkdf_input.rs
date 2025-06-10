// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::hkdf::HkdfInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput::HkdfInput {
        digestAlgorithm: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 salt: crate::standard_library_conversions::oblob_to_dafny(&value.salt),
 ikm: crate::standard_library_conversions::blob_to_dafny(&value.ikm.unwrap()),
 info: crate::standard_library_conversions::blob_to_dafny(&value.info.unwrap()),
 expectedLength: value.expected_length.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::hkdf::HkdfInput {
    crate::deps::aws_cryptography_primitives::operation::hkdf::HkdfInput::builder()
        .set_digest_algorithm(Some(
            crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::from_dafny(
                dafny_value.digestAlgorithm(),
            ),
        ))
        .set_salt(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.salt().clone(),
        ))
        .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.ikm().clone(),
        )))
        .set_info(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.info().clone(),
        )))
        .set_expected_length(Some(dafny_value.expectedLength().clone()))
        .build()
        .unwrap()
}
