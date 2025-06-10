// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndex,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndex>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndex::ReplicaGlobalSecondaryIndex {
        IndexName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.index_name),
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
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndex,
    >,
) -> aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndex {
    aws_sdk_dynamodb::types::ReplicaGlobalSecondaryIndex::builder()
          .set_index_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.IndexName()) ))
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
          .build()
          .unwrap()
}
