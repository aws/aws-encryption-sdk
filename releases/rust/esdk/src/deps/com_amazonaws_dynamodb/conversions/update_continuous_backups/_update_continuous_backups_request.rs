// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_continuous_backups::UpdateContinuousBackupsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContinuousBackupsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContinuousBackupsInput::UpdateContinuousBackupsInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 PointInTimeRecoverySpecification: crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_specification::to_dafny(&value.point_in_time_recovery_specification.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContinuousBackupsInput,
    >,
) -> aws_sdk_dynamodb::operation::update_continuous_backups::UpdateContinuousBackupsInput {
    aws_sdk_dynamodb::operation::update_continuous_backups::UpdateContinuousBackupsInput::builder()
          .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_point_in_time_recovery_specification(Some( crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_specification::from_dafny(dafny_value.PointInTimeRecoverySpecification().clone())
 ))
          .build()
          .unwrap()
}
