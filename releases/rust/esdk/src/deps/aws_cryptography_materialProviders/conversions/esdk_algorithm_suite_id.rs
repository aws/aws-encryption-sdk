// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_NO_KDF {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_NO_KDF {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_NO_KDF {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY {},
crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId,
) -> crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_NO_KDF {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_NO_KDF {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_NO_KDF {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKAlgorithmSuiteId::ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384 {} => crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
    }
}
