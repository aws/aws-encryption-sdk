// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::enable_key_rotation::EnableKeyRotationInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EnableKeyRotationRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EnableKeyRotationRequest::EnableKeyRotationRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 RotationPeriodInDays: crate::standard_library_conversions::oint_to_dafny(value.rotation_period_in_days),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EnableKeyRotationRequest,
    >,
) -> aws_sdk_kms::operation::enable_key_rotation::EnableKeyRotationInput {
    aws_sdk_kms::operation::enable_key_rotation::EnableKeyRotationInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_rotation_period_in_days(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.RotationPeriodInDays().clone(),
        ))
        .build()
        .unwrap()
}
