// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::BackupTypeFilter,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::BackupTypeFilter::User => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::USER {},
aws_sdk_dynamodb::types::BackupTypeFilter::System => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::SYSTEM {},
aws_sdk_dynamodb::types::BackupTypeFilter::AwsBackup => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::AWS_BACKUP {},
aws_sdk_dynamodb::types::BackupTypeFilter::All => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::ALL {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter,
) -> aws_sdk_dynamodb::types::BackupTypeFilter {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::USER {} => aws_sdk_dynamodb::types::BackupTypeFilter::User,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::SYSTEM {} => aws_sdk_dynamodb::types::BackupTypeFilter::System,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::AWS_BACKUP {} => aws_sdk_dynamodb::types::BackupTypeFilter::AwsBackup,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupTypeFilter::ALL {} => aws_sdk_dynamodb::types::BackupTypeFilter::All,
    }
}
