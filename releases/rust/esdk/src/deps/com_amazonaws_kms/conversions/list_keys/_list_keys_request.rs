// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::list_keys::ListKeysInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeysRequest,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeysRequest::ListKeysRequest {
        Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeysRequest,
    >
) -> aws_sdk_kms::operation::list_keys::ListKeysInput {
    aws_sdk_kms::operation::list_keys::ListKeysInput::builder()
          .set_limit(crate::standard_library_conversions::oint_from_dafny(dafny_value.Limit().clone()))
 .set_marker(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Marker().clone()))
          .build()
          .unwrap()
}
