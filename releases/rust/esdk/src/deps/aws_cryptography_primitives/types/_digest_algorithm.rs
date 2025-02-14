// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum DigestAlgorithm {
    Sha512,
    Sha384,
    Sha256,
}

impl ::std::fmt::Display for DigestAlgorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            DigestAlgorithm::Sha512 => write!(f, "SHA_512"),
            DigestAlgorithm::Sha384 => write!(f, "SHA_384"),
            DigestAlgorithm::Sha256 => write!(f, "SHA_256"),
        }
    }
}
