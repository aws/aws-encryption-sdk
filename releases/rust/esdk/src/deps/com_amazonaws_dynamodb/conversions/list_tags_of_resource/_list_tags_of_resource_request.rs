// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_tags_of_resource::ListTagsOfResourceInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTagsOfResourceInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTagsOfResourceInput::ListTagsOfResourceInput {
        ResourceArn: crate::standard_library_conversions::ostring_to_dafny(&value.resource_arn) .Extract(),
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTagsOfResourceInput,
    >,
) -> aws_sdk_dynamodb::operation::list_tags_of_resource::ListTagsOfResourceInput {
    aws_sdk_dynamodb::operation::list_tags_of_resource::ListTagsOfResourceInput::builder()
        .set_resource_arn(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.ResourceArn(),
            ),
        ))
        .set_next_token(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.NextToken().clone(),
        ))
        .build()
        .unwrap()
}
