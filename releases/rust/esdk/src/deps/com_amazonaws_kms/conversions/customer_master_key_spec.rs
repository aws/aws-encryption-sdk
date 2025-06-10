// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::CustomerMasterKeySpec,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::CustomerMasterKeySpec::Rsa2048 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_2048 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Rsa3072 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_3072 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Rsa4096 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_4096 {},
aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P256 {},
aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP384 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P384 {},
aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP521 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P521 {},
aws_sdk_kms::types::CustomerMasterKeySpec::EccSecgP256K1 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_SECG_P256K1 {},
aws_sdk_kms::types::CustomerMasterKeySpec::SymmetricDefault => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::SYMMETRIC_DEFAULT {},
aws_sdk_kms::types::CustomerMasterKeySpec::Hmac224 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_224 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Hmac256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_256 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Hmac384 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_384 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Hmac512 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_512 {},
aws_sdk_kms::types::CustomerMasterKeySpec::Sm2 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::SM2 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec,
) -> aws_sdk_kms::types::CustomerMasterKeySpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_2048 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Rsa2048,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_3072 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Rsa3072,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::RSA_4096 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Rsa4096,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P256 {} => aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P384 {} => aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP384,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_NIST_P521 {} => aws_sdk_kms::types::CustomerMasterKeySpec::EccNistP521,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::ECC_SECG_P256K1 {} => aws_sdk_kms::types::CustomerMasterKeySpec::EccSecgP256K1,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::SYMMETRIC_DEFAULT {} => aws_sdk_kms::types::CustomerMasterKeySpec::SymmetricDefault,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_224 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Hmac224,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_256 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Hmac256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_384 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Hmac384,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::HMAC_512 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Hmac512,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomerMasterKeySpec::SM2 {} => aws_sdk_kms::types::CustomerMasterKeySpec::Sm2,
    }
}
