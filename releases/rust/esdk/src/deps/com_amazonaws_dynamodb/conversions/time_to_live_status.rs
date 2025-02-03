// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::TimeToLiveStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::TimeToLiveStatus::Enabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::ENABLING {},
aws_sdk_dynamodb::types::TimeToLiveStatus::Disabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::DISABLING {},
aws_sdk_dynamodb::types::TimeToLiveStatus::Enabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::ENABLED {},
aws_sdk_dynamodb::types::TimeToLiveStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::DISABLED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus,
) -> aws_sdk_dynamodb::types::TimeToLiveStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::ENABLING {} => aws_sdk_dynamodb::types::TimeToLiveStatus::Enabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::DISABLING {} => aws_sdk_dynamodb::types::TimeToLiveStatus::Disabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::ENABLED {} => aws_sdk_dynamodb::types::TimeToLiveStatus::Enabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TimeToLiveStatus::DISABLED {} => aws_sdk_dynamodb::types::TimeToLiveStatus::Disabled,
    }
}
