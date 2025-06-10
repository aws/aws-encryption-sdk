// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AutoScalingPolicyDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyDescription::AutoScalingPolicyDescription {
        PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
 TargetTrackingScalingPolicyConfiguration: ::dafny_runtime::Rc::new(match &value.target_tracking_scaling_policy_configuration {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_target_tracking_scaling_policy_configuration_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyDescription,
    >,
) -> aws_sdk_dynamodb::types::AutoScalingPolicyDescription {
    aws_sdk_dynamodb::types::AutoScalingPolicyDescription::builder()
          .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.PolicyName().clone()))
 .set_target_tracking_scaling_policy_configuration(match (*dafny_value.TargetTrackingScalingPolicyConfiguration()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_target_tracking_scaling_policy_configuration_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
