// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ProjectionType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ProjectionType::All => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::ALL {},
aws_sdk_dynamodb::types::ProjectionType::KeysOnly => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::KEYS_ONLY {},
aws_sdk_dynamodb::types::ProjectionType::Include => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::INCLUDE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType,
) -> aws_sdk_dynamodb::types::ProjectionType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::ALL {} => aws_sdk_dynamodb::types::ProjectionType::All,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::KEYS_ONLY {} => aws_sdk_dynamodb::types::ProjectionType::KeysOnly,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType::INCLUDE {} => aws_sdk_dynamodb::types::ProjectionType::Include,
    }
}
