// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Esdk(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy::ESDK {
        ESDK: crate::deps::aws_cryptography_materialProviders::conversions::esdk_commitment_policy::to_dafny(x.clone()),
    },
crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Dbe(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy::DBE {
        DBE: crate::deps::aws_cryptography_materialProviders::conversions::dbe_commitment_policy::to_dafny(x.clone()),
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy::ESDK {
    ESDK: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Esdk(crate::deps::aws_cryptography_materialProviders::conversions::esdk_commitment_policy::from_dafny(x)),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CommitmentPolicy::DBE {
    DBE: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Dbe(crate::deps::aws_cryptography_materialProviders::conversions::dbe_commitment_policy::from_dafny(x)),
    }
}
