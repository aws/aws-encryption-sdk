// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsOutput::TransactWriteItemsOutput {
        ConsumedCapacity: ::dafny_runtime::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::consumed_capacity::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ItemCollectionMetrics:
::dafny_runtime::Rc::new(match &value.item_collection_metrics {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::item_collection_metrics::to_dafny(e)
,
)
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsOutput,
    >,
) -> aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsOutput {
    aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsOutput::builder()
          .set_consumed_capacity(match (*dafny_value.ConsumedCapacity()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>| crate::deps::com_amazonaws_dynamodb::conversions::consumed_capacity::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_item_collection_metrics(match (*dafny_value.ItemCollectionMetrics()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>| ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(v,
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>| crate::deps::com_amazonaws_dynamodb::conversions::item_collection_metrics::from_dafny(e.clone())
,
)
,
            )
        ),
    _ => None
}
)
          .build()
}
