// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ReturnValue,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ReturnValue::None => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::NONE {},
aws_sdk_dynamodb::types::ReturnValue::AllOld => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::ALL_OLD {},
aws_sdk_dynamodb::types::ReturnValue::UpdatedOld => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::UPDATED_OLD {},
aws_sdk_dynamodb::types::ReturnValue::AllNew => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::ALL_NEW {},
aws_sdk_dynamodb::types::ReturnValue::UpdatedNew => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::UPDATED_NEW {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue,
) -> aws_sdk_dynamodb::types::ReturnValue {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::NONE {} => aws_sdk_dynamodb::types::ReturnValue::None,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::ALL_OLD {} => aws_sdk_dynamodb::types::ReturnValue::AllOld,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::UPDATED_OLD {} => aws_sdk_dynamodb::types::ReturnValue::UpdatedOld,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::ALL_NEW {} => aws_sdk_dynamodb::types::ReturnValue::AllNew,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue::UPDATED_NEW {} => aws_sdk_dynamodb::types::ReturnValue::UpdatedNew,
    }
}
