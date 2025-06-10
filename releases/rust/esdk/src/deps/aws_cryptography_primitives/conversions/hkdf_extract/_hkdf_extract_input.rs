// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput::HkdfExtractInput {
        digestAlgorithm: crate::deps::aws_cryptography_primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 salt: crate::standard_library_conversions::oblob_to_dafny(&value.salt),
 ikm: crate::standard_library_conversions::blob_to_dafny(&value.ikm.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractInput {
    crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractInput::builder()
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
        .build()
        .unwrap()
}
