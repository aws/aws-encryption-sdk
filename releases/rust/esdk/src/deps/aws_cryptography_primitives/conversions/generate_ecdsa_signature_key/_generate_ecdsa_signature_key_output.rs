// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput::GenerateECDSASignatureKeyOutput {
        signatureAlgorithm: crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 verificationKey: crate::standard_library_conversions::blob_to_dafny(&value.verification_key.unwrap()),
 signingKey: crate::standard_library_conversions::blob_to_dafny(&value.signing_key.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput{
    crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput::builder()
        .set_signature_algorithm(Some( crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_verification_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.verificationKey().clone())))
 .set_signing_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signingKey().clone())))
        .build()
        .unwrap()
}
