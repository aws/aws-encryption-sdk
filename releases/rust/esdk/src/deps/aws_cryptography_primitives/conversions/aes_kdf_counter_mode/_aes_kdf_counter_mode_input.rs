// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::AesKdfCtrInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AesKdfCtrInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::AesKdfCtrInput::AesKdfCtrInput {
        ikm: crate::standard_library_conversions::blob_to_dafny(&value.ikm.unwrap()),
 expectedLength: value.expected_length.clone().unwrap(),
 nonce: crate::standard_library_conversions::oblob_to_dafny(&value.nonce),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AesKdfCtrInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::AesKdfCtrInput {
    crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::AesKdfCtrInput::builder()
        .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_expected_length(Some( dafny_value.expectedLength() .clone() ))
 .set_nonce(crate::standard_library_conversions::oblob_from_dafny(dafny_value.nonce().clone()))
        .build()
        .unwrap()
}
