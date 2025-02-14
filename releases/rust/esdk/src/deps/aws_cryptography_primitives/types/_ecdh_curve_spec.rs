// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum EcdhCurveSpec {
    EccNistP256,
    EccNistP384,
    EccNistP521,
    Sm2,
}

impl ::std::fmt::Display for EcdhCurveSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            EcdhCurveSpec::EccNistP256 => write!(f, "ECC_NIST_P256"),
            EcdhCurveSpec::EccNistP384 => write!(f, "ECC_NIST_P384"),
            EcdhCurveSpec::EccNistP521 => write!(f, "ECC_NIST_P521"),
            EcdhCurveSpec::Sm2 => write!(f, "SM2"),
        }
    }
}
