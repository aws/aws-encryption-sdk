// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ConditionalCheckFailed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ConditionalCheckFailed {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ItemCollectionSizeLimitExceeded => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ItemCollectionSizeLimitExceeded {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::RequestLimitExceeded => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::RequestLimitExceeded {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ValidationError => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ValidationError {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ProvisionedThroughputExceeded => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ProvisionedThroughputExceeded {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::TransactionConflict => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::TransactionConflict {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ThrottlingError => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ThrottlingError {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::InternalServerError => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::InternalServerError {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ResourceNotFound => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ResourceNotFound {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::AccessDenied => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::AccessDenied {},
aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::DuplicateItem => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::DuplicateItem {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum,
) -> aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ConditionalCheckFailed {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ConditionalCheckFailed,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ItemCollectionSizeLimitExceeded {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ItemCollectionSizeLimitExceeded,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::RequestLimitExceeded {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::RequestLimitExceeded,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ValidationError {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ValidationError,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ProvisionedThroughputExceeded {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ProvisionedThroughputExceeded,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::TransactionConflict {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::TransactionConflict,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ThrottlingError {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ThrottlingError,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::InternalServerError {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::InternalServerError,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::ResourceNotFound {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::ResourceNotFound,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::AccessDenied {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::AccessDenied,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementErrorCodeEnum::DuplicateItem {} => aws_sdk_dynamodb::types::BatchStatementErrorCodeEnum::DuplicateItem,
    }
}
