// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate {
        DisableScaleIn: crate::standard_library_conversions::obool_to_dafny(&value.disable_scale_in),
 ScaleInCooldown: crate::standard_library_conversions::oint_to_dafny(value.scale_in_cooldown),
 ScaleOutCooldown: crate::standard_library_conversions::oint_to_dafny(value.scale_out_cooldown),
 TargetValue: crate::standard_library_conversions::double_to_dafny(value.target_value.clone()),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
    >,
) -> aws_sdk_dynamodb::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate {
    aws_sdk_dynamodb::types::AutoScalingTargetTrackingScalingPolicyConfigurationUpdate::builder()
        .set_disable_scale_in(crate::standard_library_conversions::obool_from_dafny(
            dafny_value.DisableScaleIn().clone(),
        ))
        .set_scale_in_cooldown(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.ScaleInCooldown().clone(),
        ))
        .set_scale_out_cooldown(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.ScaleOutCooldown().clone(),
        ))
        .set_target_value(Some(
            crate::standard_library_conversions::double_from_dafny(
                &dafny_value.TargetValue().clone(),
            ),
        ))
        .build()
        .unwrap()
}
