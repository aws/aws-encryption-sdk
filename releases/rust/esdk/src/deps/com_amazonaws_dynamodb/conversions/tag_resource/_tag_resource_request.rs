// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::tag_resource::TagResourceInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TagResourceInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TagResourceInput::TagResourceInput {
        ResourceArn: crate::standard_library_conversions::ostring_to_dafny(&value.resource_arn) .Extract(),
 Tags: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.tags.clone().unwrap(),
    |e| crate::deps::com_amazonaws_dynamodb::conversions::tag::to_dafny(e)
,
)
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TagResourceInput,
    >,
) -> aws_sdk_dynamodb::operation::tag_resource::TagResourceInput {
    aws_sdk_dynamodb::operation::tag_resource::TagResourceInput::builder()
          .set_resource_arn(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.ResourceArn()) ))
 .set_tags(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.Tags(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag>| crate::deps::com_amazonaws_dynamodb::conversions::tag::from_dafny(e.clone())
,
)
 ))
          .build()
          .unwrap()
}
