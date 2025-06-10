// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::DestinationStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::DestinationStatus::Enabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ENABLING {},
aws_sdk_dynamodb::types::DestinationStatus::Active => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ACTIVE {},
aws_sdk_dynamodb::types::DestinationStatus::Disabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::DISABLING {},
aws_sdk_dynamodb::types::DestinationStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::DISABLED {},
aws_sdk_dynamodb::types::DestinationStatus::EnableFailed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ENABLE_FAILED {},
aws_sdk_dynamodb::types::DestinationStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::UPDATING {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus,
) -> aws_sdk_dynamodb::types::DestinationStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ENABLING {} => aws_sdk_dynamodb::types::DestinationStatus::Enabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ACTIVE {} => aws_sdk_dynamodb::types::DestinationStatus::Active,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::DISABLING {} => aws_sdk_dynamodb::types::DestinationStatus::Disabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::DISABLED {} => aws_sdk_dynamodb::types::DestinationStatus::Disabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::ENABLE_FAILED {} => aws_sdk_dynamodb::types::DestinationStatus::EnableFailed,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DestinationStatus::UPDATING {} => aws_sdk_dynamodb::types::DestinationStatus::Updating,
    }
}
