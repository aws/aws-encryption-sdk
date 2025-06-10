// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput::GetRSAKeyModulusLengthInput {
        publicKey: crate::standard_library_conversions::blob_to_dafny(&value.public_key.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GetRSAKeyModulusLengthInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput{
    crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput::builder()
        .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
        .build()
        .unwrap()
}
