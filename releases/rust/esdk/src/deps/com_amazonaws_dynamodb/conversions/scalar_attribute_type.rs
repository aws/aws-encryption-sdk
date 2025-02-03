// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ScalarAttributeType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ScalarAttributeType::S => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::S {},
aws_sdk_dynamodb::types::ScalarAttributeType::N => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::N {},
aws_sdk_dynamodb::types::ScalarAttributeType::B => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::B {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType,
) -> aws_sdk_dynamodb::types::ScalarAttributeType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::S {} => aws_sdk_dynamodb::types::ScalarAttributeType::S,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::N {} => aws_sdk_dynamodb::types::ScalarAttributeType::N,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType::B {} => aws_sdk_dynamodb::types::ScalarAttributeType::B,
    }
}
