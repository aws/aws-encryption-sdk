// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::InputFormat,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::InputFormat::DynamodbJson => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::DYNAMODB_JSON {},
aws_sdk_dynamodb::types::InputFormat::Ion => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::ION {},
aws_sdk_dynamodb::types::InputFormat::Csv => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::CSV {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat,
) -> aws_sdk_dynamodb::types::InputFormat {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::DYNAMODB_JSON {} => aws_sdk_dynamodb::types::InputFormat::DynamodbJson,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::ION {} => aws_sdk_dynamodb::types::InputFormat::Ion,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::InputFormat::CSV {} => aws_sdk_dynamodb::types::InputFormat::Csv,
    }
}
