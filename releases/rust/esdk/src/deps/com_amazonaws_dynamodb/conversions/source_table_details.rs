// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::SourceTableDetails,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableDetails>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableDetails::SourceTableDetails {
        TableName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.table_name),
 TableId: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.table_id),
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 TableSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.table_size_bytes),
 KeySchema: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.key_schema,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::to_dafny(e)
,
)
,
 TableCreationDateTime: crate::standard_library_conversions::timestamp_to_dafny(&value.table_creation_date_time),
 ProvisionedThroughput: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::to_dafny(&value.provisioned_throughput.clone().unwrap())
,
 OnDemandThroughput: ::dafny_runtime::Rc::new(match &value.on_demand_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ItemCount: crate::standard_library_conversions::olong_to_dafny(&value.item_count),
 BillingMode: ::dafny_runtime::Rc::new(match &value.billing_mode {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableDetails,
    >,
) -> aws_sdk_dynamodb::types::SourceTableDetails {
    aws_sdk_dynamodb::types::SourceTableDetails::builder()
          .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_table_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableId()) ))
 .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableArn().clone()))
 .set_table_size_bytes(crate::standard_library_conversions::olong_from_dafny(dafny_value.TableSizeBytes().clone()))
 .set_key_schema(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.KeySchema(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::from_dafny(e.clone())
,
)
 ))
 .set_table_creation_date_time(Some(crate::standard_library_conversions::timestamp_from_dafny(dafny_value.TableCreationDateTime().clone())))
 .set_provisioned_throughput(Some( crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::from_dafny(dafny_value.ProvisionedThroughput().clone())
 ))
 .set_on_demand_throughput(match (*dafny_value.OnDemandThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
 .set_billing_mode(match &**dafny_value.BillingMode() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
