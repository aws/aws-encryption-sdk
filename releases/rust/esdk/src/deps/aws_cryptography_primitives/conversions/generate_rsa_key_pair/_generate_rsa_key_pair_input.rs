// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput::GenerateRSAKeyPairInput {
        lengthBits: value.length_bits.clone().unwrap(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput {
    crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput::builder()
        .set_length_bits(Some( dafny_value.lengthBits() .clone() ))
        .build()
        .unwrap()
}
