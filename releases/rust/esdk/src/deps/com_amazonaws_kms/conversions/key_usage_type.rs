// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::KeyUsageType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::KeyUsageType::SignVerify => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::SIGN_VERIFY {},
aws_sdk_kms::types::KeyUsageType::EncryptDecrypt => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::ENCRYPT_DECRYPT {},
aws_sdk_kms::types::KeyUsageType::GenerateVerifyMac => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::GENERATE_VERIFY_MAC {},
aws_sdk_kms::types::KeyUsageType::KeyAgreement => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::KEY_AGREEMENT {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType,
) -> aws_sdk_kms::types::KeyUsageType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::SIGN_VERIFY {} => aws_sdk_kms::types::KeyUsageType::SignVerify,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::ENCRYPT_DECRYPT {} => aws_sdk_kms::types::KeyUsageType::EncryptDecrypt,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::GENERATE_VERIFY_MAC {} => aws_sdk_kms::types::KeyUsageType::GenerateVerifyMac,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyUsageType::KEY_AGREEMENT {} => aws_sdk_kms::types::KeyUsageType::KeyAgreement,
    }
}
