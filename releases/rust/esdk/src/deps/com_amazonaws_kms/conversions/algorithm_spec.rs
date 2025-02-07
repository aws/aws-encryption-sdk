// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::AlgorithmSpec,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::AlgorithmSpec::RsaesPkcs1V15 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_PKCS1_V1_5 {},
aws_sdk_kms::types::AlgorithmSpec::RsaesOaepSha1 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_OAEP_SHA_1 {},
aws_sdk_kms::types::AlgorithmSpec::RsaesOaepSha256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_OAEP_SHA_256 {},
aws_sdk_kms::types::AlgorithmSpec::RsaAesKeyWrapSha1 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSA_AES_KEY_WRAP_SHA_1 {},
aws_sdk_kms::types::AlgorithmSpec::RsaAesKeyWrapSha256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSA_AES_KEY_WRAP_SHA_256 {},
aws_sdk_kms::types::AlgorithmSpec::Sm2Pke => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::SM2PKE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec,
) -> aws_sdk_kms::types::AlgorithmSpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_PKCS1_V1_5 {} => aws_sdk_kms::types::AlgorithmSpec::RsaesPkcs1V15,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_OAEP_SHA_1 {} => aws_sdk_kms::types::AlgorithmSpec::RsaesOaepSha1,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSAES_OAEP_SHA_256 {} => aws_sdk_kms::types::AlgorithmSpec::RsaesOaepSha256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSA_AES_KEY_WRAP_SHA_1 {} => aws_sdk_kms::types::AlgorithmSpec::RsaAesKeyWrapSha1,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::RSA_AES_KEY_WRAP_SHA_256 {} => aws_sdk_kms::types::AlgorithmSpec::RsaAesKeyWrapSha256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AlgorithmSpec::SM2PKE {} => aws_sdk_kms::types::AlgorithmSpec::Sm2Pke,
    }
}
