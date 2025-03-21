// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExpandInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExpandInput::HkdfExpandInput {
        digestAlgorithm: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 prk: crate::standard_library_conversions::blob_to_dafny(&value.prk.unwrap()),
 info: crate::standard_library_conversions::blob_to_dafny(&value.info.unwrap()),
 expectedLength: value.expected_length.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExpandInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandInput {
    crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandInput::builder()
        .set_digest_algorithm(Some(
            crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::from_dafny(
                dafny_value.digestAlgorithm(),
            ),
        ))
        .set_prk(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.prk().clone(),
        )))
        .set_info(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.info().clone(),
        )))
        .set_expected_length(Some(dafny_value.expectedLength().clone()))
        .build()
        .unwrap()
}
