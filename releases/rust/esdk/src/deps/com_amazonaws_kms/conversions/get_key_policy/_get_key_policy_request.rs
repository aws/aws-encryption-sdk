// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_key_policy::GetKeyPolicyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyRequest::GetKeyPolicyRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyRequest,
    >,
) -> aws_sdk_kms::operation::get_key_policy::GetKeyPolicyInput {
    aws_sdk_kms::operation::get_key_policy::GetKeyPolicyInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.PolicyName().clone(),
        ))
        .build()
        .unwrap()
}
