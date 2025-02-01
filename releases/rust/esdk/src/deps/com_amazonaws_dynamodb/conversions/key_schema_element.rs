// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::KeySchemaElement,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement::KeySchemaElement {
        AttributeName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.attribute_name),
 KeyType: crate::deps::com_amazonaws_dynamodb::conversions::key_type::to_dafny(value.key_type.clone()),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement,
    >,
) -> aws_sdk_dynamodb::types::KeySchemaElement {
    aws_sdk_dynamodb::types::KeySchemaElement::builder()
        .set_attribute_name(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.AttributeName(),
            ),
        ))
        .set_key_type(Some(
            crate::deps::com_amazonaws_dynamodb::conversions::key_type::from_dafny(
                dafny_value.KeyType(),
            ),
        ))
        .build()
        .unwrap()
}
