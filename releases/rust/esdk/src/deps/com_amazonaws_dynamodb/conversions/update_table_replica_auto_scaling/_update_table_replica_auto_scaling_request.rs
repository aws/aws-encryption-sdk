// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableReplicaAutoScalingInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableReplicaAutoScalingInput::UpdateTableReplicaAutoScalingInput {
        GlobalSecondaryIndexUpdates: ::dafny_runtime::Rc::new(match &value.global_secondary_index_updates {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_auto_scaling_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 ProvisionedWriteCapacityAutoScalingUpdate: ::dafny_runtime::Rc::new(match &value.provisioned_write_capacity_auto_scaling_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaUpdates: ::dafny_runtime::Rc::new(match &value.replica_updates {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_auto_scaling_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableReplicaAutoScalingInput,
    >
) -> aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput{
    aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput::builder()
          .set_global_secondary_index_updates(match (*dafny_value.GlobalSecondaryIndexUpdates()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexAutoScalingUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_auto_scaling_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_provisioned_write_capacity_auto_scaling_update(match (*dafny_value.ProvisionedWriteCapacityAutoScalingUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_updates(match (*dafny_value.ReplicaUpdates()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaAutoScalingUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::replica_auto_scaling_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
          .unwrap()
}
