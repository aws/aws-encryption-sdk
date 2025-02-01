// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::update_primary_region::UpdatePrimaryRegionInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdatePrimaryRegionRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdatePrimaryRegionRequest::UpdatePrimaryRegionRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 PrimaryRegion: crate::standard_library_conversions::ostring_to_dafny(&value.primary_region) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdatePrimaryRegionRequest,
    >,
) -> aws_sdk_kms::operation::update_primary_region::UpdatePrimaryRegionInput {
    aws_sdk_kms::operation::update_primary_region::UpdatePrimaryRegionInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_primary_region(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.PrimaryRegion(),
            ),
        ))
        .build()
        .unwrap()
}
