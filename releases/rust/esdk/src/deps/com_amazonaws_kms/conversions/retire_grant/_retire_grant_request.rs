// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::retire_grant::RetireGrantInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RetireGrantRequest,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RetireGrantRequest::RetireGrantRequest {
        GrantToken: crate::standard_library_conversions::ostring_to_dafny(&value.grant_token),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 GrantId: crate::standard_library_conversions::ostring_to_dafny(&value.grant_id),
 DryRun: crate::standard_library_conversions::obool_to_dafny(&value.dry_run),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RetireGrantRequest,
    >
) -> aws_sdk_kms::operation::retire_grant::RetireGrantInput {
    aws_sdk_kms::operation::retire_grant::RetireGrantInput::builder()
          .set_grant_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantToken().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_grant_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantId().clone()))
 .set_dry_run(crate::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
          .build()
          .unwrap()
}
