// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AutoScalingSettingsDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsDescription::AutoScalingSettingsDescription {
        MinimumUnits: crate::standard_library_conversions::olong_to_dafny(&value.minimum_units),
 MaximumUnits: crate::standard_library_conversions::olong_to_dafny(&value.maximum_units),
 AutoScalingDisabled: crate::standard_library_conversions::obool_to_dafny(&value.auto_scaling_disabled),
 AutoScalingRoleArn: crate::standard_library_conversions::ostring_to_dafny(&value.auto_scaling_role_arn),
 ScalingPolicies: ::dafny_runtime::Rc::new(match &value.scaling_policies {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_policy_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingSettingsDescription,
    >,
) -> aws_sdk_dynamodb::types::AutoScalingSettingsDescription {
    aws_sdk_dynamodb::types::AutoScalingSettingsDescription::builder()
          .set_minimum_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.MinimumUnits().clone()))
 .set_maximum_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.MaximumUnits().clone()))
 .set_auto_scaling_disabled(crate::standard_library_conversions::obool_from_dafny(dafny_value.AutoScalingDisabled().clone()))
 .set_auto_scaling_role_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.AutoScalingRoleArn().clone()))
 .set_scaling_policies(match (*dafny_value.ScalingPolicies()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AutoScalingPolicyDescription>| crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_policy_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
