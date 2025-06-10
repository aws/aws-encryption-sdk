// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AutoScalingPolicyUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyUpdate::AutoScalingPolicyUpdate {
        PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
 TargetTrackingScalingPolicyConfiguration: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_target_tracking_scaling_policy_configuration_update::to_dafny(&value.target_tracking_scaling_policy_configuration.clone().unwrap())
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyUpdate,
    >,
) -> aws_sdk_dynamodb::types::AutoScalingPolicyUpdate {
    aws_sdk_dynamodb::types::AutoScalingPolicyUpdate::builder()
          .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.PolicyName().clone()))
 .set_target_tracking_scaling_policy_configuration(Some( crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_target_tracking_scaling_policy_configuration_update::from_dafny(dafny_value.TargetTrackingScalingPolicyConfiguration().clone())
 ))
          .build()
}
