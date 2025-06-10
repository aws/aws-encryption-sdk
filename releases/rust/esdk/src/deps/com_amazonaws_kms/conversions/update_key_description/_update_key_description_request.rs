// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::update_key_description::UpdateKeyDescriptionInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateKeyDescriptionRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateKeyDescriptionRequest::UpdateKeyDescriptionRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 Description: crate::standard_library_conversions::ostring_to_dafny(&value.description) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateKeyDescriptionRequest,
    >,
) -> aws_sdk_kms::operation::update_key_description::UpdateKeyDescriptionInput {
    aws_sdk_kms::operation::update_key_description::UpdateKeyDescriptionInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_description(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.Description(),
            ),
        ))
        .build()
        .unwrap()
}
