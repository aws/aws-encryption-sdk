// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::PointInTimeRecoveryStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Enabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus::ENABLED {},
aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus::DISABLED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus,
) -> aws_sdk_dynamodb::types::PointInTimeRecoveryStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus::ENABLED {} => aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Enabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryStatus::DISABLED {} => aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Disabled,
    }
}
