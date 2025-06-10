// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ImportStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ImportStatus::InProgress => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::IN_PROGRESS {},
aws_sdk_dynamodb::types::ImportStatus::Completed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::COMPLETED {},
aws_sdk_dynamodb::types::ImportStatus::Cancelling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::CANCELLING {},
aws_sdk_dynamodb::types::ImportStatus::Cancelled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::CANCELLED {},
aws_sdk_dynamodb::types::ImportStatus::Failed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::FAILED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus,
) -> aws_sdk_dynamodb::types::ImportStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::IN_PROGRESS {} => aws_sdk_dynamodb::types::ImportStatus::InProgress,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::COMPLETED {} => aws_sdk_dynamodb::types::ImportStatus::Completed,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::CANCELLING {} => aws_sdk_dynamodb::types::ImportStatus::Cancelling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::CANCELLED {} => aws_sdk_dynamodb::types::ImportStatus::Cancelled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportStatus::FAILED {} => aws_sdk_dynamodb::types::ImportStatus::Failed,
    }
}
