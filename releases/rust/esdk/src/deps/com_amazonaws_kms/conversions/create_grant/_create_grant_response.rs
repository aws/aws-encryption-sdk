// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::create_grant::CreateGrantOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantResponse::CreateGrantResponse {
        GrantToken: crate::standard_library_conversions::ostring_to_dafny(&value.grant_token),
 GrantId: crate::standard_library_conversions::ostring_to_dafny(&value.grant_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantResponse,
    >,
) -> aws_sdk_kms::operation::create_grant::CreateGrantOutput {
    aws_sdk_kms::operation::create_grant::CreateGrantOutput::builder()
        .set_grant_token(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.GrantToken().clone(),
        ))
        .set_grant_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.GrantId().clone(),
        ))
        .build()
}
