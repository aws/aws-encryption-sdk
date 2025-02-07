// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ConditionalOperator,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ConditionalOperator::And => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator::AND {},
aws_sdk_dynamodb::types::ConditionalOperator::Or => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator::OR {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator,
) -> aws_sdk_dynamodb::types::ConditionalOperator {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator::AND {} => aws_sdk_dynamodb::types::ConditionalOperator::And,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator::OR {} => aws_sdk_dynamodb::types::ConditionalOperator::Or,
    }
}
