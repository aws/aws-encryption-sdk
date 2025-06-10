// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::GlobalSecondaryIndex,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex::GlobalSecondaryIndex {
        IndexName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.index_name),
 KeySchema: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.key_schema,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::to_dafny(e)
,
)
,
 Projection: crate::deps::com_amazonaws_dynamodb::conversions::projection::to_dafny(&value.projection.clone().unwrap())
,
 ProvisionedThroughput: ::dafny_runtime::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex,
    >,
) -> aws_sdk_dynamodb::types::GlobalSecondaryIndex {
    aws_sdk_dynamodb::types::GlobalSecondaryIndex::builder()
          .set_index_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.IndexName()) ))
 .set_key_schema(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.KeySchema(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::from_dafny(e.clone())
,
)
 ))
 .set_projection(Some( crate::deps::com_amazonaws_dynamodb::conversions::projection::from_dafny(dafny_value.Projection().clone())
 ))
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::provisioned_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_on_demand_throughput(match (*dafny_value.OnDemandThroughput()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::on_demand_throughput::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
