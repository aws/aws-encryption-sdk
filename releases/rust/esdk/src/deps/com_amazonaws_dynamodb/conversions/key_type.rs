// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::KeyType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::KeyType::Hash => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType::HASH {},
aws_sdk_dynamodb::types::KeyType::Range => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType::RANGE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType,
) -> aws_sdk_dynamodb::types::KeyType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType::HASH {} => aws_sdk_dynamodb::types::KeyType::Hash,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType::RANGE {} => aws_sdk_dynamodb::types::KeyType::Range,
    }
}
