// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::DbeCommitmentPolicy,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBECommitmentPolicy>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::DbeCommitmentPolicy::RequireEncryptRequireDecrypt => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBECommitmentPolicy::REQUIRE_ENCRYPT_REQUIRE_DECRYPT {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBECommitmentPolicy,
) -> crate::deps::aws_cryptography_materialProviders::types::DbeCommitmentPolicy {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBECommitmentPolicy::REQUIRE_ENCRYPT_REQUIRE_DECRYPT {} => crate::deps::aws_cryptography_materialProviders::types::DbeCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}
