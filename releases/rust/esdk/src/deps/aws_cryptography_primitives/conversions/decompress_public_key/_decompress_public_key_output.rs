// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyOutput::DecompressPublicKeyOutput {
        publicKey: crate::deps::aws_cryptography_primitives::conversions::ecc_public_key::to_dafny(&value.public_key.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyOutput{
    crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyOutput::builder()
        .set_public_key(Some( crate::deps::aws_cryptography_primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
        .build()
        .unwrap()
}
