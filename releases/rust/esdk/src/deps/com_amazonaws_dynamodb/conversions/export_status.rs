// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ExportStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ExportStatus::InProgress => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::IN_PROGRESS {},
aws_sdk_dynamodb::types::ExportStatus::Completed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::COMPLETED {},
aws_sdk_dynamodb::types::ExportStatus::Failed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::FAILED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus,
) -> aws_sdk_dynamodb::types::ExportStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::IN_PROGRESS {} => aws_sdk_dynamodb::types::ExportStatus::InProgress,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::COMPLETED {} => aws_sdk_dynamodb::types::ExportStatus::Completed,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportStatus::FAILED {} => aws_sdk_dynamodb::types::ExportStatus::Failed,
    }
}
