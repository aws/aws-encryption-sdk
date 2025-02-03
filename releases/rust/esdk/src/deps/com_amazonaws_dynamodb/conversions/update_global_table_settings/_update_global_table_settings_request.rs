// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_global_table_settings::UpdateGlobalTableSettingsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableSettingsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableSettingsInput::UpdateGlobalTableSettingsInput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name) .Extract(),
 GlobalTableBillingMode: ::dafny_runtime::Rc::new(match &value.global_table_billing_mode {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalTableProvisionedWriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.global_table_provisioned_write_capacity_units),
 GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate: ::dafny_runtime::Rc::new(match &value.global_table_provisioned_write_capacity_auto_scaling_settings_update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GlobalTableGlobalSecondaryIndexSettingsUpdate: ::dafny_runtime::Rc::new(match &value.global_table_global_secondary_index_settings_update {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_table_global_secondary_index_settings_update::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ReplicaSettingsUpdate: ::dafny_runtime::Rc::new(match &value.replica_settings_update {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::replica_settings_update::to_dafny(e)
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateGlobalTableSettingsInput,
    >,
) -> aws_sdk_dynamodb::operation::update_global_table_settings::UpdateGlobalTableSettingsInput {
    aws_sdk_dynamodb::operation::update_global_table_settings::UpdateGlobalTableSettingsInput::builder()
          .set_global_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.GlobalTableName()) ))
 .set_global_table_billing_mode(match &**dafny_value.GlobalTableBillingMode() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
 .set_global_table_provisioned_write_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.GlobalTableProvisionedWriteCapacityUnits().clone()))
 .set_global_table_provisioned_write_capacity_auto_scaling_settings_update(match (*dafny_value.GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::auto_scaling_settings_update::from_dafny(value.clone())),
    _ => None,
}
)
 .set_global_table_global_secondary_index_settings_update(match (*dafny_value.GlobalTableGlobalSecondaryIndexSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::global_table_global_secondary_index_settings_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_replica_settings_update(match (*dafny_value.ReplicaSettingsUpdate()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaSettingsUpdate>| crate::deps::com_amazonaws_dynamodb::conversions::replica_settings_update::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
          .unwrap()
}
