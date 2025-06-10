// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AttributeValueUpdate,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValueUpdate>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValueUpdate::AttributeValueUpdate {
        Value: ::dafny_runtime::Rc::new(match &value.value {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Action: ::dafny_runtime::Rc::new(match &value.action {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::attribute_action::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValueUpdate,
    >,
) -> aws_sdk_dynamodb::types::AttributeValueUpdate {
    aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
        .set_value(match (*dafny_value.Value()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .set_action(match &**dafny_value.Action() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::attribute_action::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
}
