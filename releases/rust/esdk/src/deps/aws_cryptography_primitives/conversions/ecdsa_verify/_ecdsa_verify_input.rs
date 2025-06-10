// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSAVerifyInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSAVerifyInput::ECDSAVerifyInput {
        signatureAlgorithm: crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 verificationKey: crate::standard_library_conversions::blob_to_dafny(&value.verification_key.unwrap()),
 message: crate::standard_library_conversions::blob_to_dafny(&value.message.unwrap()),
 signature: crate::standard_library_conversions::blob_to_dafny(&value.signature.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSAVerifyInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput {
    crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput::builder()
        .set_signature_algorithm(Some( crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_verification_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.verificationKey().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
 .set_signature(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signature().clone())))
        .build()
        .unwrap()
}
