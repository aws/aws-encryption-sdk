// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TransactWriteItem,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem::TransactWriteItem {
        ConditionCheck: ::std::rc::Rc::new(match &value.condition_check {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::condition_check::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Put: ::std::rc::Rc::new(match &value.put {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::put::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Delete: ::std::rc::Rc::new(match &value.delete {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::delete::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Update: ::std::rc::Rc::new(match &value.update {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::update::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem,
    >,
) -> aws_sdk_dynamodb::types::TransactWriteItem {
    aws_sdk_dynamodb::types::TransactWriteItem::builder()
          .set_condition_check(match (*dafny_value.ConditionCheck()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::condition_check::from_dafny(value.clone())),
    _ => None,
}
)
 .set_put(match (*dafny_value.Put()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::put::from_dafny(value.clone())),
    _ => None,
}
)
 .set_delete(match (*dafny_value.Delete()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::delete::from_dafny(value.clone())),
    _ => None,
}
)
 .set_update(match (*dafny_value.Update()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::update::from_dafny(value.clone())),
    _ => None,
}
)
          .build()

}
