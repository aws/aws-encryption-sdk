// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::PointInTimeRecoveryDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryDescription::PointInTimeRecoveryDescription {
        PointInTimeRecoveryStatus: ::dafny_runtime::Rc::new(match &value.point_in_time_recovery_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 EarliestRestorableDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.earliest_restorable_date_time),
 LatestRestorableDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.latest_restorable_date_time),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PointInTimeRecoveryDescription,
    >,
) -> aws_sdk_dynamodb::types::PointInTimeRecoveryDescription {
    aws_sdk_dynamodb::types::PointInTimeRecoveryDescription::builder()
          .set_point_in_time_recovery_status(match &**dafny_value.PointInTimeRecoveryStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::point_in_time_recovery_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_earliest_restorable_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.EarliestRestorableDateTime().clone()))
 .set_latest_restorable_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.LatestRestorableDateTime().clone()))
          .build()
}
