// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum DbeAlgorithmSuiteId {
    AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384,
    AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384,
}

impl ::std::fmt::Display for DbeAlgorithmSuiteId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => write!(
                f,
                "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384"
            ),
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => {
                write!(
                    f,
                    "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384"
                )
            }
        }
    }
}
