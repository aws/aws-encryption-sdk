// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsUpdate::ReplicaGlobalSecondaryIndexSettingsUpdate {
        IndexName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.index_name),
 ProvisionedReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.provisioned_read_capacity_units),
 ProvisionedReadCapacityAutoScalingSettingsUpdate: ::dafny_runtime::Rc::new(match &value.provisioned_read_capacity_auto_scaling_settings_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsUpdate,
    >,
) -> aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsUpdate {
    aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndexSettingsUpdate::builder()
          .set_index_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.IndexName()) ))
 .set_provisioned_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ProvisionedReadCapacityUnits().clone()))
 .set_provisioned_read_capacity_auto_scaling_settings_update(match (*dafny_value.ProvisionedReadCapacityAutoScalingSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
