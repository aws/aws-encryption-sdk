// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ReturnConsumedCapacity,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ReturnConsumedCapacity::Indexes => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::INDEXES {},
aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::TOTAL {},
aws_sdk_dynamodb::types::ReturnConsumedCapacity::None => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::NONE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity,
) -> aws_sdk_dynamodb::types::ReturnConsumedCapacity {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::INDEXES {} => aws_sdk_dynamodb::types::ReturnConsumedCapacity::Indexes,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::TOTAL {} => aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity::NONE {} => aws_sdk_dynamodb::types::ReturnConsumedCapacity::None,
    }
}
