// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ExpectedAttributeValue,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue::ExpectedAttributeValue {
        Value: ::dafny_runtime::Rc::new(match &value.value {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Exists: crate::standard_library_conversions::obool_to_dafny(&value.exists),
 ComparisonOperator: ::dafny_runtime::Rc::new(match &value.comparison_operator {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::comparison_operator::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 AttributeValueList: ::dafny_runtime::Rc::new(match &value.attribute_value_list {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue,
    >,
) -> aws_sdk_dynamodb::types::ExpectedAttributeValue {
    aws_sdk_dynamodb::types::ExpectedAttributeValue::builder()
          .set_value(match (*dafny_value.Value()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(value.clone())),
    _ => None,
}
)
 .set_exists(crate::standard_library_conversions::obool_from_dafny(dafny_value.Exists().clone()))
 .set_comparison_operator(match &**dafny_value.ComparisonOperator() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::comparison_operator::from_dafny(value)
    ),
    _ => None,
}
)
 .set_attribute_value_list(match (*dafny_value.AttributeValueList()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
