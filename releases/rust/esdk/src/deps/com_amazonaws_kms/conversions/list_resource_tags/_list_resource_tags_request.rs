// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::list_resource_tags::ListResourceTagsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListResourceTagsRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListResourceTagsRequest::ListResourceTagsRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListResourceTagsRequest,
    >,
) -> aws_sdk_kms::operation::list_resource_tags::ListResourceTagsInput {
    aws_sdk_kms::operation::list_resource_tags::ListResourceTagsInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_limit(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.Limit().clone(),
        ))
        .set_marker(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Marker().clone(),
        ))
        .build()
        .unwrap()
}
