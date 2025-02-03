// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::S3SseAlgorithm,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::S3SseAlgorithm::Aes256 => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm::AES256 {},
aws_sdk_dynamodb::types::S3SseAlgorithm::Kms => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm::KMS {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm,
) -> aws_sdk_dynamodb::types::S3SseAlgorithm {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm::AES256 {} => aws_sdk_dynamodb::types::S3SseAlgorithm::Aes256,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3SseAlgorithm::KMS {} => aws_sdk_dynamodb::types::S3SseAlgorithm::Kms,
    }
}
