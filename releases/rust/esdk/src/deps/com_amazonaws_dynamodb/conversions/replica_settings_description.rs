// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaSettingsDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsDescription::ReplicaSettingsDescription {
        RegionName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.region_name),
 ReplicaStatus: ::dafny_runtime::Rc::new(match &value.replica_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::replica_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaBillingModeSummary: ::dafny_runtime::Rc::new(match &value.replica_billing_mode_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaProvisionedReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.replica_provisioned_read_capacity_units),
 ReplicaProvisionedReadCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.replica_provisioned_read_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaProvisionedWriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.replica_provisioned_write_capacity_units),
 ReplicaProvisionedWriteCapacityAutoScalingSettings: ::dafny_runtime::Rc::new(match &value.replica_provisioned_write_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaGlobalSecondaryIndexSettings: ::dafny_runtime::Rc::new(match &value.replica_global_secondary_index_settings {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_settings_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaTableClassSummary: ::dafny_runtime::Rc::new(match &value.replica_table_class_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_class_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsDescription,
    >,
) -> aws_sdk_dynamodb::types::ReplicaSettingsDescription {
    aws_sdk_dynamodb::types::ReplicaSettingsDescription::builder()
          .set_region_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.RegionName()) ))
 .set_replica_status(match &**dafny_value.ReplicaStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::replica_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_replica_billing_mode_summary(match (*dafny_value.ReplicaBillingModeSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::billing_mode_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_provisioned_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ReplicaProvisionedReadCapacityUnits().clone()))
 .set_replica_provisioned_read_capacity_auto_scaling_settings(match (*dafny_value.ReplicaProvisionedReadCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_provisioned_write_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ReplicaProvisionedWriteCapacityUnits().clone()))
 .set_replica_provisioned_write_capacity_auto_scaling_settings(match (*dafny_value.ReplicaProvisionedWriteCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_global_secondary_index_settings(match (*dafny_value.ReplicaGlobalSecondaryIndexSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexSettingsDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_settings_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_table_class_summary(match (*dafny_value.ReplicaTableClassSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::table_class_summary::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
