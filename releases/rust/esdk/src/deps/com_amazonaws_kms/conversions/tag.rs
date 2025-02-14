// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::Tag,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Tag,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Tag::Tag {
        TagKey: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.tag_key),
 TagValue: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.tag_value),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Tag,
    >,
) -> aws_sdk_kms::types::Tag {
    aws_sdk_kms::types::Tag::builder()
        .set_tag_key(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.TagKey(),
            ),
        ))
        .set_tag_value(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.TagValue(),
            ),
        ))
        .build()
        .unwrap()
}
