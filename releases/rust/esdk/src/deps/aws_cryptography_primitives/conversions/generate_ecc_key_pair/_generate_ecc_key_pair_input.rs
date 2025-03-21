// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECCKeyPairInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECCKeyPairInput::GenerateECCKeyPairInput {
        eccCurve: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateECCKeyPairInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput{
    crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput::builder()
        .set_ecc_curve(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
