// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::BackupType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::BackupType::User => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::USER {},
aws_sdk_dynamodb::types::BackupType::System => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::SYSTEM {},
aws_sdk_dynamodb::types::BackupType::AwsBackup => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::AWS_BACKUP {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType,
) -> aws_sdk_dynamodb::types::BackupType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::USER {} => aws_sdk_dynamodb::types::BackupType::User,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::SYSTEM {} => aws_sdk_dynamodb::types::BackupType::System,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupType::AWS_BACKUP {} => aws_sdk_dynamodb::types::BackupType::AwsBackup,
    }
}
