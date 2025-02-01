// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ComparisonOperator,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ComparisonOperator::Eq => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::EQ {},
aws_sdk_dynamodb::types::ComparisonOperator::Ne => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NE {},
aws_sdk_dynamodb::types::ComparisonOperator::In => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::IN {},
aws_sdk_dynamodb::types::ComparisonOperator::Le => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::LE {},
aws_sdk_dynamodb::types::ComparisonOperator::Lt => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::LT {},
aws_sdk_dynamodb::types::ComparisonOperator::Ge => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::GE {},
aws_sdk_dynamodb::types::ComparisonOperator::Gt => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::GT {},
aws_sdk_dynamodb::types::ComparisonOperator::Between => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::BETWEEN {},
aws_sdk_dynamodb::types::ComparisonOperator::NotNull => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NOT_NULL {},
aws_sdk_dynamodb::types::ComparisonOperator::Null => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NULL {},
aws_sdk_dynamodb::types::ComparisonOperator::Contains => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::CONTAINS {},
aws_sdk_dynamodb::types::ComparisonOperator::NotContains => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NOT_CONTAINS {},
aws_sdk_dynamodb::types::ComparisonOperator::BeginsWith => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::BEGINS_WITH {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator,
) -> aws_sdk_dynamodb::types::ComparisonOperator {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::EQ {} => aws_sdk_dynamodb::types::ComparisonOperator::Eq,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NE {} => aws_sdk_dynamodb::types::ComparisonOperator::Ne,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::IN {} => aws_sdk_dynamodb::types::ComparisonOperator::In,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::LE {} => aws_sdk_dynamodb::types::ComparisonOperator::Le,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::LT {} => aws_sdk_dynamodb::types::ComparisonOperator::Lt,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::GE {} => aws_sdk_dynamodb::types::ComparisonOperator::Ge,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::GT {} => aws_sdk_dynamodb::types::ComparisonOperator::Gt,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::BETWEEN {} => aws_sdk_dynamodb::types::ComparisonOperator::Between,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NOT_NULL {} => aws_sdk_dynamodb::types::ComparisonOperator::NotNull,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NULL {} => aws_sdk_dynamodb::types::ComparisonOperator::Null,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::CONTAINS {} => aws_sdk_dynamodb::types::ComparisonOperator::Contains,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::NOT_CONTAINS {} => aws_sdk_dynamodb::types::ComparisonOperator::NotContains,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator::BEGINS_WITH {} => aws_sdk_dynamodb::types::ComparisonOperator::BeginsWith,
    }
}
