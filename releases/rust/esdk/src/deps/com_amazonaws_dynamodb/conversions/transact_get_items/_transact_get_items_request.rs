// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput::TransactGetItemsInput {
        TransactItems: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.transact_items.clone().unwrap(),
    |e| crate::deps::com_amazonaws_dynamodb::conversions::transact_get_item::to_dafny(e)
,
)
,
 ReturnConsumedCapacity: ::dafny_runtime::Rc::new(match &value.return_consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::return_consumed_capacity::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput,
    >,
) -> aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsInput {
    aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsInput::builder()
          .set_transact_items(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.TransactItems(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItem>| crate::deps::com_amazonaws_dynamodb::conversions::transact_get_item::from_dafny(e.clone())
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
          .build()
          .unwrap()
}
