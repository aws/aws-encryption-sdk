// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput::GenerateRSAKeyPairOutput {
        publicKey: crate::deps::aws_cryptography_primitives::conversions::rsa_public_key::to_dafny(&value.public_key.clone().unwrap())
,
 privateKey: crate::deps::aws_cryptography_primitives::conversions::rsa_private_key::to_dafny(&value.private_key.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput{
    crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput::builder()
        .set_public_key(Some( crate::deps::aws_cryptography_primitives::conversions::rsa_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
 .set_private_key(Some( crate::deps::aws_cryptography_primitives::conversions::rsa_private_key::from_dafny(dafny_value.privateKey().clone())
 ))
        .build()
        .unwrap()
}
