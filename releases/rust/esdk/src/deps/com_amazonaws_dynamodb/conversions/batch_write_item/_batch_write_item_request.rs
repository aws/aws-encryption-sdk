// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::batch_write_item::BatchWriteItemInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchWriteItemInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchWriteItemInput::BatchWriteItemInput {
        RequestItems: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.request_items.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::write_request::to_dafny(e)
,
)
,
)
,
 ReturnConsumedCapacity: ::dafny_runtime::Rc::new(match &value.return_consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::return_consumed_capacity::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReturnItemCollectionMetrics: ::dafny_runtime::Rc::new(match &value.return_item_collection_metrics {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::return_item_collection_metrics::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchWriteItemInput,
    >,
) -> aws_sdk_dynamodb::operation::batch_write_item::BatchWriteItemInput {
    aws_sdk_dynamodb::operation::batch_write_item::BatchWriteItemInput::builder()
          .set_request_items(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.RequestItems(),
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::WriteRequest>>| ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(v,
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::WriteRequest>| crate::deps::com_amazonaws_dynamodb::conversions::write_request::from_dafny(e.clone())
,
)
,
)
 ))
 .set_return_consumed_capacity(match &**dafny_value.ReturnConsumedCapacity() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::return_consumed_capacity::from_dafny(value)
    ),
    _ => None,
}
)
 .set_return_item_collection_metrics(match &**dafny_value.ReturnItemCollectionMetrics() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::return_item_collection_metrics::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
