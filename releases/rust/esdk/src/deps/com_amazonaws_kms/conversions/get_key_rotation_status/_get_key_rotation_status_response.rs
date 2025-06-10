// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_key_rotation_status::GetKeyRotationStatusOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyRotationStatusResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyRotationStatusResponse::GetKeyRotationStatusResponse {
        KeyRotationEnabled: crate::standard_library_conversions::obool_to_dafny(&Some(value.key_rotation_enabled)),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 RotationPeriodInDays: crate::standard_library_conversions::oint_to_dafny(value.rotation_period_in_days),
 NextRotationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.next_rotation_date),
 OnDemandRotationStartDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.on_demand_rotation_start_date),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyRotationStatusResponse,
    >,
) -> aws_sdk_kms::operation::get_key_rotation_status::GetKeyRotationStatusOutput {
    aws_sdk_kms::operation::get_key_rotation_status::GetKeyRotationStatusOutput::builder()
        .set_key_rotation_enabled(crate::standard_library_conversions::obool_from_dafny(
            dafny_value.KeyRotationEnabled().clone(),
        ))
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_rotation_period_in_days(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.RotationPeriodInDays().clone(),
        ))
        .set_next_rotation_date(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.NextRotationDate().clone(),
        ))
        .set_on_demand_rotation_start_date(
            crate::standard_library_conversions::otimestamp_from_dafny(
                dafny_value.OnDemandRotationStartDate().clone(),
            ),
        )
        .build()
}
