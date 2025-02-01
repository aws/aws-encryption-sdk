// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput::CompressPublicKeyInput {
        publicKey: crate::deps::aws_cryptography_primitives::conversions::ecc_public_key::to_dafny(&value.public_key.clone().unwrap())
,
 eccCurve: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::CompressPublicKeyInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput
{
    crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput::builder()
        .set_public_key(Some( crate::deps::aws_cryptography_primitives::conversions::ecc_public_key::from_dafny(dafny_value.publicKey().clone())
 ))
 .set_ecc_curve(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
