// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::PointInTimeRecoverySpecification,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoverySpecification>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoverySpecification::PointInTimeRecoverySpecification {
        PointInTimeRecoveryEnabled: value.point_in_time_recovery_enabled.clone(),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoverySpecification,
    >,
) -> aws_sdk_dynamodb::types::PointInTimeRecoverySpecification {
    aws_sdk_dynamodb::types::PointInTimeRecoverySpecification::builder()
        .set_point_in_time_recovery_enabled(Some(dafny_value.PointInTimeRecoveryEnabled().clone()))
        .build()
        .unwrap()
}
