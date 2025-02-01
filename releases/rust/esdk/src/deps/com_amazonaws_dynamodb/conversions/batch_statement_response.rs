// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BatchStatementResponse,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementResponse>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementResponse::BatchStatementResponse {
        Error: ::dafny_runtime::Rc::new(match &value.error {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::batch_statement_error::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 Item:
::dafny_runtime::Rc::new(match &value.item {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(v)
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementResponse,
    >,
) -> aws_sdk_dynamodb::types::BatchStatementResponse {
    aws_sdk_dynamodb::types::BatchStatementResponse::builder()
          .set_error(match (*dafny_value.Error()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::batch_statement_error::from_dafny(value.clone())),
    _ => None,
}
)
 .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_item(match (*dafny_value.Item()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
