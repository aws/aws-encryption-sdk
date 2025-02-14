// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput::DecompressPublicKeyInput {
        compressedPublicKey: crate::standard_library_conversions::blob_to_dafny(&value.compressed_public_key.unwrap()),
 eccCurve: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyInput{
    crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyInput::builder()
        .set_compressed_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.compressedPublicKey().clone())))
 .set_ecc_curve(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
