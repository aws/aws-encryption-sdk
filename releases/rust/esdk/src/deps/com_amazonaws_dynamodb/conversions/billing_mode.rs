// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::BillingMode,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::BillingMode::Provisioned => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode::PROVISIONED {},
aws_sdk_dynamodb::types::BillingMode::PayPerRequest => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode::PAY_PER_REQUEST {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode,
) -> aws_sdk_dynamodb::types::BillingMode {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode::PROVISIONED {} => aws_sdk_dynamodb::types::BillingMode::Provisioned,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode::PAY_PER_REQUEST {} => aws_sdk_dynamodb::types::BillingMode::PayPerRequest,
    }
}
