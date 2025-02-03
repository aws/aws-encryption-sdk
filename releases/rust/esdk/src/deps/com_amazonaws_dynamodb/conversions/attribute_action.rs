// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::AttributeAction,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::AttributeAction::Add => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::ADD {},
aws_sdk_dynamodb::types::AttributeAction::Put => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::PUT {},
aws_sdk_dynamodb::types::AttributeAction::Delete => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::DELETE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction,
) -> aws_sdk_dynamodb::types::AttributeAction {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::ADD {} => aws_sdk_dynamodb::types::AttributeAction::Add,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::PUT {} => aws_sdk_dynamodb::types::AttributeAction::Put,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction::DELETE {} => aws_sdk_dynamodb::types::AttributeAction::Delete,
    }
}
