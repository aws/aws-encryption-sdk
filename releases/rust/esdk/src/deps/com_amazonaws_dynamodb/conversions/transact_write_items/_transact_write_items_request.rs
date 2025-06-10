// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsInput::TransactWriteItemsInput {
        TransactItems: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.transact_items.clone().unwrap(),
    |e| crate::deps::com_amazonaws_dynamodb::conversions::transact_write_item::to_dafny(e)
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
 ClientRequestToken: crate::standard_library_conversions::ostring_to_dafny(&value.client_request_token),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsInput,
    >,
) -> aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsInput {
    aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsInput::builder()
          .set_transact_items(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.TransactItems(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem>| crate::deps::com_amazonaws_dynamodb::conversions::transact_write_item::from_dafny(e.clone())
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
 .set_client_request_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ClientRequestToken().clone()))
          .build()
          .unwrap()
}
