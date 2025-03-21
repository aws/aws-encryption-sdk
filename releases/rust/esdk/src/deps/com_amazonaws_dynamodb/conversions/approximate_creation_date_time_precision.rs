// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision::Millisecond => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision::MILLISECOND {},
aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision::Microsecond => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision::MICROSECOND {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision,
) -> aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision::MILLISECOND {} => aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision::Millisecond,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ApproximateCreationDateTimePrecision::MICROSECOND {} => aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision::Microsecond,
    }
}
