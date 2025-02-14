// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ContinuousBackupsStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus::ENABLED {},
aws_sdk_dynamodb::types::ContinuousBackupsStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus::DISABLED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus,
) -> aws_sdk_dynamodb::types::ContinuousBackupsStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus::ENABLED {} => aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsStatus::DISABLED {} => aws_sdk_dynamodb::types::ContinuousBackupsStatus::Disabled,
    }
}
