// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::put_key_policy::PutKeyPolicyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::PutKeyPolicyRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::PutKeyPolicyRequest::PutKeyPolicyRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
 Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy) .Extract(),
 BypassPolicyLockoutSafetyCheck: crate::standard_library_conversions::obool_to_dafny(&value.bypass_policy_lockout_safety_check),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::PutKeyPolicyRequest,
    >,
) -> aws_sdk_kms::operation::put_key_policy::PutKeyPolicyInput {
    aws_sdk_kms::operation::put_key_policy::PutKeyPolicyInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.PolicyName().clone(),
        ))
        .set_policy(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.Policy(),
            ),
        ))
        .set_bypass_policy_lockout_safety_check(
            crate::standard_library_conversions::obool_from_dafny(
                dafny_value.BypassPolicyLockoutSafetyCheck().clone(),
            ),
        )
        .build()
        .unwrap()
}
