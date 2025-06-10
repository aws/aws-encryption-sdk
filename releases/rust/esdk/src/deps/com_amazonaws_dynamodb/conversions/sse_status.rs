// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::SseStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::SseStatus::Enabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::ENABLING {},
aws_sdk_dynamodb::types::SseStatus::Enabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::ENABLED {},
aws_sdk_dynamodb::types::SseStatus::Disabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::DISABLING {},
aws_sdk_dynamodb::types::SseStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::DISABLED {},
aws_sdk_dynamodb::types::SseStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::UPDATING {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus,
) -> aws_sdk_dynamodb::types::SseStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::ENABLING {} => aws_sdk_dynamodb::types::SseStatus::Enabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::ENABLED {} => aws_sdk_dynamodb::types::SseStatus::Enabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::DISABLING {} => aws_sdk_dynamodb::types::SseStatus::Disabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::DISABLED {} => aws_sdk_dynamodb::types::SseStatus::Disabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus::UPDATING {} => aws_sdk_dynamodb::types::SseStatus::Updating,
    }
}
