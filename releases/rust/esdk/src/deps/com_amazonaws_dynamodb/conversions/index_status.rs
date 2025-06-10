// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::IndexStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::IndexStatus::Creating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::CREATING {},
aws_sdk_dynamodb::types::IndexStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::UPDATING {},
aws_sdk_dynamodb::types::IndexStatus::Deleting => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::DELETING {},
aws_sdk_dynamodb::types::IndexStatus::Active => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::ACTIVE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus,
) -> aws_sdk_dynamodb::types::IndexStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::CREATING {} => aws_sdk_dynamodb::types::IndexStatus::Creating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::UPDATING {} => aws_sdk_dynamodb::types::IndexStatus::Updating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::DELETING {} => aws_sdk_dynamodb::types::IndexStatus::Deleting,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus::ACTIVE {} => aws_sdk_dynamodb::types::IndexStatus::Active,
    }
}
