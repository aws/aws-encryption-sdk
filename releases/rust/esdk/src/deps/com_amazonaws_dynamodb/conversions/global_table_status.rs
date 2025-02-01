// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::GlobalTableStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::GlobalTableStatus::Creating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::CREATING {},
aws_sdk_dynamodb::types::GlobalTableStatus::Active => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::ACTIVE {},
aws_sdk_dynamodb::types::GlobalTableStatus::Deleting => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::DELETING {},
aws_sdk_dynamodb::types::GlobalTableStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::UPDATING {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus,
) -> aws_sdk_dynamodb::types::GlobalTableStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::CREATING {} => aws_sdk_dynamodb::types::GlobalTableStatus::Creating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::ACTIVE {} => aws_sdk_dynamodb::types::GlobalTableStatus::Active,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::DELETING {} => aws_sdk_dynamodb::types::GlobalTableStatus::Deleting,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableStatus::UPDATING {} => aws_sdk_dynamodb::types::GlobalTableStatus::Updating,
    }
}
