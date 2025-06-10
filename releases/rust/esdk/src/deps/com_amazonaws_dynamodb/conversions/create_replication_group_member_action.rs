// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::CreateReplicationGroupMemberAction,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateReplicationGroupMemberAction>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateReplicationGroupMemberAction::CreateReplicationGroupMemberAction {
        RegionName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.region_name),
 KMSMasterKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.kms_master_key_id),
 ProvisionedThroughputOverride: ::dafny_runtime::Rc::new(match &value.provisioned_throughput_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput_override::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 OnDemandThroughputOverride: ::dafny_runtime::Rc::new(match &value.on_demand_throughput_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput_override::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalSecondaryIndexes: ::dafny_runtime::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 TableClassOverride: ::dafny_runtime::Rc::new(match &value.table_class_override {
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateReplicationGroupMemberAction,
    >,
) -> aws_sdk_dynamodb::types::CreateReplicationGroupMemberAction {
    aws_sdk_dynamodb::types::CreateReplicationGroupMemberAction::builder()
          .set_region_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.RegionName()) ))
 .set_kms_master_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KMSMasterKeyId().clone()))
 .set_provisioned_throughput_override(match (*dafny_value.ProvisionedThroughputOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput_override::from_dafny(value.clone())),
    _ => None,
}
)
 .set_on_demand_throughput_override(match (*dafny_value.OnDemandThroughputOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput_override::from_dafny(value.clone())),
    _ => None,
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndex>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_class_override(match &**dafny_value.TableClassOverride() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::table_class::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
