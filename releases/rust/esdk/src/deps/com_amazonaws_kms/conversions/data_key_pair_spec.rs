// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::DataKeyPairSpec,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::DataKeyPairSpec::Rsa2048 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_2048 {},
aws_sdk_kms::types::DataKeyPairSpec::Rsa3072 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_3072 {},
aws_sdk_kms::types::DataKeyPairSpec::Rsa4096 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_4096 {},
aws_sdk_kms::types::DataKeyPairSpec::EccNistP256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P256 {},
aws_sdk_kms::types::DataKeyPairSpec::EccNistP384 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P384 {},
aws_sdk_kms::types::DataKeyPairSpec::EccNistP521 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P521 {},
aws_sdk_kms::types::DataKeyPairSpec::EccSecgP256K1 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_SECG_P256K1 {},
aws_sdk_kms::types::DataKeyPairSpec::Sm2 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::SM2 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec,
) -> aws_sdk_kms::types::DataKeyPairSpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_2048 {} => aws_sdk_kms::types::DataKeyPairSpec::Rsa2048,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_3072 {} => aws_sdk_kms::types::DataKeyPairSpec::Rsa3072,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::RSA_4096 {} => aws_sdk_kms::types::DataKeyPairSpec::Rsa4096,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P256 {} => aws_sdk_kms::types::DataKeyPairSpec::EccNistP256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P384 {} => aws_sdk_kms::types::DataKeyPairSpec::EccNistP384,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_NIST_P521 {} => aws_sdk_kms::types::DataKeyPairSpec::EccNistP521,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::ECC_SECG_P256K1 {} => aws_sdk_kms::types::DataKeyPairSpec::EccSecgP256K1,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DataKeyPairSpec::SM2 {} => aws_sdk_kms::types::DataKeyPairSpec::Sm2,
    }
}
