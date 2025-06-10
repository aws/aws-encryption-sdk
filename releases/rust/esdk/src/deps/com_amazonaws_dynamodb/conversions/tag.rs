// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::Tag,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag::Tag {
        Key: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.key),
 Value: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.value),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag,
    >,
) -> aws_sdk_dynamodb::types::Tag {
    aws_sdk_dynamodb::types::Tag::builder()
        .set_key(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.Key(),
            ),
        ))
        .set_value(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.Value(),
            ),
        ))
        .build()
        .unwrap()
}
