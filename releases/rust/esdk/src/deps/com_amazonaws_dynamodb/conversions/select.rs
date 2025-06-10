// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::Select,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::Select::AllAttributes => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::ALL_ATTRIBUTES {},
aws_sdk_dynamodb::types::Select::AllProjectedAttributes => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::ALL_PROJECTED_ATTRIBUTES {},
aws_sdk_dynamodb::types::Select::SpecificAttributes => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::SPECIFIC_ATTRIBUTES {},
aws_sdk_dynamodb::types::Select::Count => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::COUNT {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select,
) -> aws_sdk_dynamodb::types::Select {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::ALL_ATTRIBUTES {} => aws_sdk_dynamodb::types::Select::AllAttributes,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::ALL_PROJECTED_ATTRIBUTES {} => aws_sdk_dynamodb::types::Select::AllProjectedAttributes,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::SPECIFIC_ATTRIBUTES {} => aws_sdk_dynamodb::types::Select::SpecificAttributes,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Select::COUNT {} => aws_sdk_dynamodb::types::Select::Count,
    }
}
