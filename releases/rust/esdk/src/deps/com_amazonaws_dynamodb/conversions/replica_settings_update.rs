// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaSettingsUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsUpdate::ReplicaSettingsUpdate {
        RegionName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.region_name),
 ReplicaProvisionedReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.replica_provisioned_read_capacity_units),
 ReplicaProvisionedReadCapacityAutoScalingSettingsUpdate: ::dafny_runtime::Rc::new(match &value.replica_provisioned_read_capacity_auto_scaling_settings_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaGlobalSecondaryIndexSettingsUpdate: ::dafny_runtime::Rc::new(match &value.replica_global_secondary_index_settings_update {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_settings_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaTableClass: ::dafny_runtime::Rc::new(match &value.replica_table_class {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_class::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsUpdate,
    >,
) -> aws_sdk_dynamodb::types::ReplicaSettingsUpdate {
    aws_sdk_dynamodb::types::ReplicaSettingsUpdate::builder()
          .set_region_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.RegionName()) ))
 .set_replica_provisioned_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ReplicaProvisionedReadCapacityUnits().clone()))
 .set_replica_provisioned_read_capacity_auto_scaling_settings_update(match (*dafny_value.ReplicaProvisionedReadCapacityAutoScalingSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_global_secondary_index_settings_update(match (*dafny_value.ReplicaGlobalSecondaryIndexSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_settings_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_table_class(match &**dafny_value.ReplicaTableClass() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::table_class::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
