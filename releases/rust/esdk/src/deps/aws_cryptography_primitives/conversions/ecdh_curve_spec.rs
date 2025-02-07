// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP256 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P256 {},
crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP384 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P384 {},
crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP521 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P521 {},
crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::Sm2 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::SM2 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec,
) -> crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P256 {} => crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP256,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P384 {} => crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP384,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::ECC_NIST_P521 {} => crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::EccNistP521,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec::SM2 {} => crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec::Sm2,
    }
}
