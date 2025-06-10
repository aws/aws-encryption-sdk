// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ContinuousBackupsDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsDescription::ContinuousBackupsDescription {
        ContinuousBackupsStatus: crate::deps::com_amazonaws_dynamodb::conversions::continuous_backups_status::to_dafny(value.continuous_backups_status.clone()),
 PointInTimeRecoveryDescription: ::dafny_runtime::Rc::new(match &value.point_in_time_recovery_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContinuousBackupsDescription,
    >,
) -> aws_sdk_dynamodb::types::ContinuousBackupsDescription {
    aws_sdk_dynamodb::types::ContinuousBackupsDescription::builder()
          .set_continuous_backups_status(Some( crate::deps::com_amazonaws_dynamodb::conversions::continuous_backups_status::from_dafny(dafny_value.ContinuousBackupsStatus()) ))
 .set_point_in_time_recovery_description(match (*dafny_value.PointInTimeRecoveryDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
