// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::GrantOperation,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::GrantOperation::Decrypt => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Decrypt {},
aws_sdk_kms::types::GrantOperation::Encrypt => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Encrypt {},
aws_sdk_kms::types::GrantOperation::GenerateDataKey => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKey {},
aws_sdk_kms::types::GrantOperation::GenerateDataKeyWithoutPlaintext => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyWithoutPlaintext {},
aws_sdk_kms::types::GrantOperation::ReEncryptFrom => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::ReEncryptFrom {},
aws_sdk_kms::types::GrantOperation::ReEncryptTo => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::ReEncryptTo {},
aws_sdk_kms::types::GrantOperation::Sign => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Sign {},
aws_sdk_kms::types::GrantOperation::Verify => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Verify {},
aws_sdk_kms::types::GrantOperation::GetPublicKey => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GetPublicKey {},
aws_sdk_kms::types::GrantOperation::CreateGrant => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::CreateGrant {},
aws_sdk_kms::types::GrantOperation::RetireGrant => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::RetireGrant {},
aws_sdk_kms::types::GrantOperation::DescribeKey => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::DescribeKey {},
aws_sdk_kms::types::GrantOperation::GenerateDataKeyPair => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyPair {},
aws_sdk_kms::types::GrantOperation::GenerateDataKeyPairWithoutPlaintext => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyPairWithoutPlaintext {},
aws_sdk_kms::types::GrantOperation::GenerateMac => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateMac {},
aws_sdk_kms::types::GrantOperation::VerifyMac => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::VerifyMac {},
aws_sdk_kms::types::GrantOperation::DeriveSharedSecret => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::DeriveSharedSecret {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation,
) -> aws_sdk_kms::types::GrantOperation {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Decrypt {} => aws_sdk_kms::types::GrantOperation::Decrypt,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Encrypt {} => aws_sdk_kms::types::GrantOperation::Encrypt,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKey {} => aws_sdk_kms::types::GrantOperation::GenerateDataKey,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyWithoutPlaintext {} => aws_sdk_kms::types::GrantOperation::GenerateDataKeyWithoutPlaintext,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::ReEncryptFrom {} => aws_sdk_kms::types::GrantOperation::ReEncryptFrom,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::ReEncryptTo {} => aws_sdk_kms::types::GrantOperation::ReEncryptTo,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Sign {} => aws_sdk_kms::types::GrantOperation::Sign,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::Verify {} => aws_sdk_kms::types::GrantOperation::Verify,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GetPublicKey {} => aws_sdk_kms::types::GrantOperation::GetPublicKey,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::CreateGrant {} => aws_sdk_kms::types::GrantOperation::CreateGrant,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::RetireGrant {} => aws_sdk_kms::types::GrantOperation::RetireGrant,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::DescribeKey {} => aws_sdk_kms::types::GrantOperation::DescribeKey,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyPair {} => aws_sdk_kms::types::GrantOperation::GenerateDataKeyPair,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateDataKeyPairWithoutPlaintext {} => aws_sdk_kms::types::GrantOperation::GenerateDataKeyPairWithoutPlaintext,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::GenerateMac {} => aws_sdk_kms::types::GrantOperation::GenerateMac,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::VerifyMac {} => aws_sdk_kms::types::GrantOperation::VerifyMac,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GrantOperation::DeriveSharedSecret {} => aws_sdk_kms::types::GrantOperation::DeriveSharedSecret,
    }
}
