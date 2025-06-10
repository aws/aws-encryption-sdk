// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::TableClass,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::TableClass::Standard => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass::STANDARD {},
aws_sdk_dynamodb::types::TableClass::StandardInfrequentAccess => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass::STANDARD_INFREQUENT_ACCESS {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass,
) -> aws_sdk_dynamodb::types::TableClass {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass::STANDARD {} => aws_sdk_dynamodb::types::TableClass::Standard,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass::STANDARD_INFREQUENT_ACCESS {} => aws_sdk_dynamodb::types::TableClass::StandardInfrequentAccess,
    }
}
