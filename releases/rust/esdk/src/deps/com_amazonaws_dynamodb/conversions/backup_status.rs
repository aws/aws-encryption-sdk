// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::BackupStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::BackupStatus::Creating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::CREATING {},
aws_sdk_dynamodb::types::BackupStatus::Deleted => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::DELETED {},
aws_sdk_dynamodb::types::BackupStatus::Available => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::AVAILABLE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus,
) -> aws_sdk_dynamodb::types::BackupStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::CREATING {} => aws_sdk_dynamodb::types::BackupStatus::Creating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::DELETED {} => aws_sdk_dynamodb::types::BackupStatus::Deleted,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupStatus::AVAILABLE {} => aws_sdk_dynamodb::types::BackupStatus::Available,
    }
}
