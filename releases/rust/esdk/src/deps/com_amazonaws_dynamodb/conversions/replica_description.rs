// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription::ReplicaDescription {
        RegionName: crate::standard_library_conversions::ostring_to_dafny(&value.region_name),
 ReplicaStatus: ::dafny_runtime::Rc::new(match &value.replica_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::replica_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaStatusDescription: crate::standard_library_conversions::ostring_to_dafny(&value.replica_status_description),
 ReplicaStatusPercentProgress: crate::standard_library_conversions::ostring_to_dafny(&value.replica_status_percent_progress),
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
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaInaccessibleDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.replica_inaccessible_date_time),
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription,
    >,
) -> aws_sdk_dynamodb::types::ReplicaDescription {
    aws_sdk_dynamodb::types::ReplicaDescription::builder()
          .set_region_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RegionName().clone()))
 .set_replica_status(match &**dafny_value.ReplicaStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::replica_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_replica_status_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ReplicaStatusDescription().clone()))
 .set_replica_status_percent_progress(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ReplicaStatusPercentProgress().clone()))
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
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_global_secondary_index_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_inaccessible_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.ReplicaInaccessibleDateTime().clone()))
 .set_replica_table_class_summary(match (*dafny_value.ReplicaTableClassSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::table_class_summary::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
