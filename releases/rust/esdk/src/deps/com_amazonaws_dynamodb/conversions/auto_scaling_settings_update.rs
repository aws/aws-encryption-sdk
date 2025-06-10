// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AutoScalingSettingsUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsUpdate::AutoScalingSettingsUpdate {
        MinimumUnits: crate::standard_library_conversions::olong_to_dafny(&value.minimum_units),
 MaximumUnits: crate::standard_library_conversions::olong_to_dafny(&value.maximum_units),
 AutoScalingDisabled: crate::standard_library_conversions::obool_to_dafny(&value.auto_scaling_disabled),
 AutoScalingRoleArn: crate::standard_library_conversions::ostring_to_dafny(&value.auto_scaling_role_arn),
 ScalingPolicyUpdate: ::dafny_runtime::Rc::new(match &value.scaling_policy_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_policy_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsUpdate,
    >,
) -> aws_sdk_dynamodb::types::AutoScalingSettingsUpdate {
    aws_sdk_dynamodb::types::AutoScalingSettingsUpdate::builder()
          .set_minimum_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.MinimumUnits().clone()))
 .set_maximum_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.MaximumUnits().clone()))
 .set_auto_scaling_disabled(crate::standard_library_conversions::obool_from_dafny(dafny_value.AutoScalingDisabled().clone()))
 .set_auto_scaling_role_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.AutoScalingRoleArn().clone()))
 .set_scaling_policy_update(match (*dafny_value.ScalingPolicyUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_policy_update::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
