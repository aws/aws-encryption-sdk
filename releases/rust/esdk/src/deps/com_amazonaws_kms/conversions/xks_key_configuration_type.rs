// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::XksKeyConfigurationType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksKeyConfigurationType>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksKeyConfigurationType::XksKeyConfigurationType {
        Id: crate::standard_library_conversions::ostring_to_dafny(&value.id),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksKeyConfigurationType,
    >,
) -> aws_sdk_kms::types::XksKeyConfigurationType {
    aws_sdk_kms::types::XksKeyConfigurationType::builder()
        .set_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Id().clone(),
        ))
        .build()
}
