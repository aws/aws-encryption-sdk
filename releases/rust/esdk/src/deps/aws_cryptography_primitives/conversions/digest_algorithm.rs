// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha512 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_512 {},
crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha384 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_384 {},
crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha256 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_256 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm,
) -> crate::deps::aws_cryptography_primitives::types::DigestAlgorithm {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_512 {} => crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha512,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_384 {} => crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha384,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm::SHA_256 {} => crate::deps::aws_cryptography_primitives::types::DigestAlgorithm::Sha256,
    }
}
