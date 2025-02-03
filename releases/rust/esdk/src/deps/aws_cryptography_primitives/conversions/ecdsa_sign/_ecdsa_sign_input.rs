// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput::ECDSASignInput {
        signatureAlgorithm: crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 signingKey: crate::standard_library_conversions::blob_to_dafny(&value.signing_key.unwrap()),
 message: crate::standard_library_conversions::blob_to_dafny(&value.message.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput {
    crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput::builder()
        .set_signature_algorithm(Some( crate::deps::aws_cryptography_primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_signing_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signingKey().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
        .build()
        .unwrap()
}
