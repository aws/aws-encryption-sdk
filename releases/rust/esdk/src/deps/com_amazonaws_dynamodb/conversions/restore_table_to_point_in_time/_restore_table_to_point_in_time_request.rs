// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableToPointInTimeInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableToPointInTimeInput::RestoreTableToPointInTimeInput {
        SourceTableArn: crate::standard_library_conversions::ostring_to_dafny(&value.source_table_arn),
 SourceTableName: crate::standard_library_conversions::ostring_to_dafny(&value.source_table_name),
 TargetTableName: crate::standard_library_conversions::ostring_to_dafny(&value.target_table_name) .Extract(),
 UseLatestRestorableTime: crate::standard_library_conversions::obool_to_dafny(&value.use_latest_restorable_time),
 RestoreDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.restore_date_time),
 BillingModeOverride: ::dafny_runtime::Rc::new(match &value.billing_mode_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalSecondaryIndexOverride: ::dafny_runtime::Rc::new(match &value.global_secondary_index_override {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 LocalSecondaryIndexOverride: ::dafny_runtime::Rc::new(match &value.local_secondary_index_override {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ProvisionedThroughputOverride: ::dafny_runtime::Rc::new(match &value.provisioned_throughput_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 OnDemandThroughputOverride: ::dafny_runtime::Rc::new(match &value.on_demand_throughput_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SSESpecificationOverride: ::dafny_runtime::Rc::new(match &value.sse_specification_override {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreTableToPointInTimeInput,
    >,
) -> aws_sdk_dynamodb::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput {
    aws_sdk_dynamodb::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput::builder()
          .set_source_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.SourceTableArn().clone()))
 .set_source_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.SourceTableName().clone()))
 .set_target_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TargetTableName()) ))
 .set_use_latest_restorable_time(crate::standard_library_conversions::obool_from_dafny(dafny_value.UseLatestRestorableTime().clone()))
 .set_restore_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.RestoreDateTime().clone()))
 .set_billing_mode_override(match &**dafny_value.BillingModeOverride() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
 .set_global_secondary_index_override(match (*dafny_value.GlobalSecondaryIndexOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex>| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_local_secondary_index_override(match (*dafny_value.LocalSecondaryIndexOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndex>| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_provisioned_throughput_override(match (*dafny_value.ProvisionedThroughputOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_on_demand_throughput_override(match (*dafny_value.OnDemandThroughputOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_sse_specification_override(match (*dafny_value.SSESpecificationOverride()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_specification::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
