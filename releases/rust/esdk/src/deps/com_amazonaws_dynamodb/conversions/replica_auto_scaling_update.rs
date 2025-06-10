// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaAutoScalingUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingUpdate::ReplicaAutoScalingUpdate {
        RegionName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.region_name),
 ReplicaGlobalSecondaryIndexUpdates: ::dafny_runtime::Rc::new(match &value.replica_global_secondary_index_updates {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_auto_scaling_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaProvisionedReadCapacityAutoScalingUpdate: ::dafny_runtime::Rc::new(match &value.replica_provisioned_read_capacity_auto_scaling_update {
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingUpdate,
    >,
) -> aws_sdk_dynamodb::types::ReplicaAutoScalingUpdate {
    aws_sdk_dynamodb::types::ReplicaAutoScalingUpdate::builder()
          .set_region_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.RegionName()) ))
 .set_replica_global_secondary_index_updates(match (*dafny_value.ReplicaGlobalSecondaryIndexUpdates()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexAutoScalingUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_auto_scaling_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_provisioned_read_capacity_auto_scaling_update(match (*dafny_value.ReplicaProvisionedReadCapacityAutoScalingUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
