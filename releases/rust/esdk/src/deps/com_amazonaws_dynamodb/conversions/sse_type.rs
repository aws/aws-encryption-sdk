// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::SseType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::SseType::Aes256 => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType::AES256 {},
aws_sdk_dynamodb::types::SseType::Kms => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType::KMS {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType,
) -> aws_sdk_dynamodb::types::SseType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType::AES256 {} => aws_sdk_dynamodb::types::SseType::Aes256,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType::KMS {} => aws_sdk_dynamodb::types::SseType::Kms,
    }
}
