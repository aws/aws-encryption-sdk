// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaAutoScalingDescription,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingDescription>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingDescription::ReplicaAutoScalingDescription {
        RegionName: crate::standard_library_conversions::ostring_to_dafny(&value.region_name),
 GlobalSecondaryIndexes: ::std::rc::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_auto_scaling_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaProvisionedReadCapacityAutoScalingSettings: ::std::rc::Rc::new(match &value.replica_provisioned_read_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaProvisionedWriteCapacityAutoScalingSettings: ::std::rc::Rc::new(match &value.replica_provisioned_write_capacity_auto_scaling_settings {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaStatus: ::std::rc::Rc::new(match &value.replica_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::replica_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingDescription,
    >,
) -> aws_sdk_dynamodb::types::ReplicaAutoScalingDescription {
    aws_sdk_dynamodb::types::ReplicaAutoScalingDescription::builder()
          .set_region_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RegionName().clone()))
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexAutoScalingDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_auto_scaling_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_provisioned_read_capacity_auto_scaling_settings(match (*dafny_value.ReplicaProvisionedReadCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_provisioned_write_capacity_auto_scaling_settings(match (*dafny_value.ReplicaProvisionedWriteCapacityAutoScalingSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_status(match &**dafny_value.ReplicaStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::replica_status::from_dafny(value)
    ),
    _ => None,
}
)
          .build()

}
