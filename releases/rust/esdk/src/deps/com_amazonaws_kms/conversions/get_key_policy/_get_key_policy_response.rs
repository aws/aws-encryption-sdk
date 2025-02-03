// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_key_policy::GetKeyPolicyOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyResponse::GetKeyPolicyResponse {
        Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy),
 PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyResponse,
    >,
) -> aws_sdk_kms::operation::get_key_policy::GetKeyPolicyOutput {
    aws_sdk_kms::operation::get_key_policy::GetKeyPolicyOutput::builder()
        .set_policy(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Policy().clone(),
        ))
        .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.PolicyName().clone(),
        ))
        .build()
}
