// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum EsdkAlgorithmSuiteId {
    AlgAes128GcmIv12Tag16NoKdf,
    AlgAes192GcmIv12Tag16NoKdf,
    AlgAes256GcmIv12Tag16NoKdf,
    AlgAes128GcmIv12Tag16HkdfSha256,
    AlgAes192GcmIv12Tag16HkdfSha256,
    AlgAes256GcmIv12Tag16HkdfSha256,
    AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
    AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
    AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
    AlgAes256GcmHkdfSha512CommitKey,
    AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
}

impl ::std::fmt::Display for EsdkAlgorithmSuiteId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => {
                write!(f, "ALG_AES_128_GCM_IV12_TAG16_NO_KDF")
            }
            EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => {
                write!(f, "ALG_AES_192_GCM_IV12_TAG16_NO_KDF")
            }
            EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => {
                write!(f, "ALG_AES_256_GCM_IV12_TAG16_NO_KDF")
            }
            EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
                write!(f, "ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256")
            }
            EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
                write!(f, "ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256")
            }
            EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
                write!(f, "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256")
            }
            EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
                write!(f, "ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256")
            }
            EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
                write!(f, "ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")
            }
            EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
                write!(f, "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")
            }
            EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
                write!(f, "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY")
            }
            EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
                write!(f, "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384")
            }
        }
    }
}
