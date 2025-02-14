// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum EcdsaSignatureAlgorithm {
    EcdsaP384,
    EcdsaP256,
}

impl ::std::fmt::Display for EcdsaSignatureAlgorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            EcdsaSignatureAlgorithm::EcdsaP384 => write!(f, "ECDSA_P384"),
            EcdsaSignatureAlgorithm::EcdsaP256 => write!(f, "ECDSA_P256"),
        }
    }
}
