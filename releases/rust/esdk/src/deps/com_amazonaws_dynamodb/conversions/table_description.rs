// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TableDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription::TableDescription {
        AttributeDefinitions: ::dafny_runtime::Rc::new(match &value.attribute_definitions {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::attribute_definition::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 KeySchema: ::dafny_runtime::Rc::new(match &value.key_schema {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 TableStatus: ::dafny_runtime::Rc::new(match &value.table_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CreationDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date_time),
 ProvisionedThroughput: ::dafny_runtime::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.table_size_bytes),
 ItemCount: crate::standard_library_conversions::olong_to_dafny(&value.item_count),
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 TableId: crate::standard_library_conversions::ostring_to_dafny(&value.table_id),
 BillingModeSummary: ::dafny_runtime::Rc::new(match &value.billing_mode_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LocalSecondaryIndexes: ::dafny_runtime::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 GlobalSecondaryIndexes: ::dafny_runtime::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 StreamSpecification: ::dafny_runtime::Rc::new(match &value.stream_specification {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::stream_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LatestStreamLabel: crate::standard_library_conversions::ostring_to_dafny(&value.latest_stream_label),
 LatestStreamArn: crate::standard_library_conversions::ostring_to_dafny(&value.latest_stream_arn),
 GlobalTableVersion: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_version),
 Replicas: ::dafny_runtime::Rc::new(match &value.replicas {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_description::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 RestoreSummary: ::dafny_runtime::Rc::new(match &value.restore_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::restore_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SSEDescription: ::dafny_runtime::Rc::new(match &value.sse_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ArchivalSummary: ::dafny_runtime::Rc::new(match &value.archival_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::archival_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableClassSummary: ::dafny_runtime::Rc::new(match &value.table_class_summary {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_class_summary::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DeletionProtectionEnabled: crate::standard_library_conversions::obool_to_dafny(&value.deletion_protection_enabled),
 OnDemandThroughput: ::dafny_runtime::Rc::new(match &value.on_demand_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription,
    >,
) -> aws_sdk_dynamodb::types::TableDescription {
    aws_sdk_dynamodb::types::TableDescription::builder()
          .set_attribute_definitions(match (*dafny_value.AttributeDefinitions()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeDefinition>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_definition::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_key_schema(match (*dafny_value.KeySchema()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_status(match &**dafny_value.TableStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::table_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_creation_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDateTime().clone()))
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_table_size_bytes(crate::standard_library_conversions::olong_from_dafny(dafny_value.TableSizeBytes().clone()))
 .set_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
 .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableArn().clone()))
 .set_table_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableId().clone()))
 .set_billing_mode_summary(match (*dafny_value.BillingModeSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::billing_mode_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription>| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription>| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_stream_specification(match (*dafny_value.StreamSpecification()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::stream_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_latest_stream_label(crate::standard_library_conversions::ostring_from_dafny(dafny_value.LatestStreamLabel().clone()))
 .set_latest_stream_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.LatestStreamArn().clone()))
 .set_global_table_version(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableVersion().clone()))
 .set_replicas(match (*dafny_value.Replicas()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription>| crate::deps::com_amazonaws_dynamodb::conversions::replica_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_restore_summary(match (*dafny_value.RestoreSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::restore_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_sse_description(match (*dafny_value.SSEDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_archival_summary(match (*dafny_value.ArchivalSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::archival_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_table_class_summary(match (*dafny_value.TableClassSummary()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::table_class_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_deletion_protection_enabled(crate::standard_library_conversions::obool_from_dafny(dafny_value.DeletionProtectionEnabled().clone()))
 .set_on_demand_throughput(match (*dafny_value.OnDemandThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
