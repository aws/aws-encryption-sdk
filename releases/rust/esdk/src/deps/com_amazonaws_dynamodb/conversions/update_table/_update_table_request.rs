// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_table::UpdateTableInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableInput::UpdateTableInput {
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
 TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 BillingMode: ::dafny_runtime::Rc::new(match &value.billing_mode {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ProvisionedThroughput: ::dafny_runtime::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalSecondaryIndexUpdates: ::dafny_runtime::Rc::new(match &value.global_secondary_index_updates {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_update::to_dafny(e)
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
 SSESpecification: ::dafny_runtime::Rc::new(match &value.sse_specification {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaUpdates: ::dafny_runtime::Rc::new(match &value.replica_updates {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replication_group_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 TableClass: ::dafny_runtime::Rc::new(match &value.table_class {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_class::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DeletionProtectionEnabled: crate::standard_library_conversions::obool_to_dafny(&value.deletion_protection_enabled),
 OnDemandThroughput: ::dafny_runtime::Rc::new(match &value.on_demand_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateTableInput,
    >,
) -> aws_sdk_dynamodb::operation::update_table::UpdateTableInput {
    aws_sdk_dynamodb::operation::update_table::UpdateTableInput::builder()
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
 .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_billing_mode(match &**dafny_value.BillingMode() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_global_secondary_index_updates(match (*dafny_value.GlobalSecondaryIndexUpdates()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_update::from_dafny(e.clone())
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
 .set_sse_specification(match (*dafny_value.SSESpecification()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_updates(match (*dafny_value.ReplicaUpdates()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicationGroupUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::replication_group_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_class(match &**dafny_value.TableClass() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::table_class::from_dafny(value)
    ),
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
          .unwrap()
}
