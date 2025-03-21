// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput::GenerateRSAKeyPairInput {
        lengthBits: value.length_bits.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput{
    crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput::builder()
        .set_length_bits(Some( dafny_value.lengthBits() .clone() ))
        .build()
        .unwrap()
}
