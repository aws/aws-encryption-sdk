// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyInput::PutResourcePolicyInput {
        ResourceArn: crate::standard_library_conversions::ostring_to_dafny(&value.resource_arn) .Extract(),
 Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy) .Extract(),
 ExpectedRevisionId: crate::standard_library_conversions::ostring_to_dafny(&value.expected_revision_id),
 ConfirmRemoveSelfResourceAccess: crate::standard_library_conversions::obool_to_dafny(&value.confirm_remove_self_resource_access),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyInput,
    >,
) -> aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyInput {
    aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyInput::builder()
        .set_resource_arn(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.ResourceArn(),
            ),
        ))
        .set_policy(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.Policy(),
            ),
        ))
        .set_expected_revision_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ExpectedRevisionId().clone(),
        ))
        .set_confirm_remove_self_resource_access(
            crate::standard_library_conversions::obool_from_dafny(
                dafny_value.ConfirmRemoveSelfResourceAccess().clone(),
            ),
        )
        .build()
        .unwrap()
}
