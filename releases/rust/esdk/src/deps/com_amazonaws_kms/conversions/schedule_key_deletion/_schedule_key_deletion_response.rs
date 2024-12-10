// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::schedule_key_deletion::ScheduleKeyDeletionOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ScheduleKeyDeletionResponse,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ScheduleKeyDeletionResponse::ScheduleKeyDeletionResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 DeletionDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.deletion_date),
 KeyState: ::std::rc::Rc::new(match &value.key_state {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_state::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 PendingWindowInDays: crate::standard_library_conversions::oint_to_dafny(value.pending_window_in_days),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ScheduleKeyDeletionResponse,
    >
) -> aws_sdk_kms::operation::schedule_key_deletion::ScheduleKeyDeletionOutput {
    aws_sdk_kms::operation::schedule_key_deletion::ScheduleKeyDeletionOutput::builder()
          .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_deletion_date(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.DeletionDate().clone()))
 .set_key_state(match &**dafny_value.KeyState() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_state::from_dafny(value)
    ),
    _ => None,
}
)
 .set_pending_window_in_days(crate::standard_library_conversions::oint_from_dafny(dafny_value.PendingWindowInDays().clone()))
          .build()


}
