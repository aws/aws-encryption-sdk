// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384 {},
crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId,
) -> crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384 {} => crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DBEAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 {} => crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384,
    }
}
