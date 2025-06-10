// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsDescription::ReplicaGlobalSecondaryIndexSettingsDescription {
        IndexName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.index_name),
 IndexStatus: ::dafny_runtime::Rc::new(match &value.index_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::index_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ProvisionedReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.provisioned_read_capacity_units),
 ProvisionedReadCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.provisioned_read_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ProvisionedWriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.provisioned_write_capacity_units),
 ProvisionedWriteCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.provisioned_write_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsDescription,
    >,
) -> aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsDescription {
    aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsDescription::builder()
          .set_index_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.IndexName()) ))
 .set_index_status(match &**dafny_value.IndexStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::index_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_provisioned_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ProvisionedReadCapacityUnits().clone()))
 .set_provisioned_read_capacity_auto_scaling_settings(match (*dafny_value.ProvisionedReadCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_provisioned_write_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ProvisionedWriteCapacityUnits().clone()))
 .set_provisioned_write_capacity_auto_scaling_settings(match (*dafny_value.ProvisionedWriteCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
