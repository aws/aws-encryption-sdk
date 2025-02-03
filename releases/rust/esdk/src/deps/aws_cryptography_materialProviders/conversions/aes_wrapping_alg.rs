// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes128GcmIv12Tag16 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES128_GCM_IV12_TAG16 {},
crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes192GcmIv12Tag16 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES192_GCM_IV12_TAG16 {},
crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes256GcmIv12Tag16 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES256_GCM_IV12_TAG16 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg,
) -> crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES128_GCM_IV12_TAG16 {} => crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes128GcmIv12Tag16,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES192_GCM_IV12_TAG16 {} => crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes192GcmIv12Tag16,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AesWrappingAlg::ALG_AES256_GCM_IV12_TAG16 {} => crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg::AlgAes256GcmIv12Tag16,
    }
}
