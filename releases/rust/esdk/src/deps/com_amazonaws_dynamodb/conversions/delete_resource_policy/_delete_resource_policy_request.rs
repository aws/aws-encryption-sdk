// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyInput::DeleteResourcePolicyInput {
        ResourceArn: crate::standard_library_conversions::ostring_to_dafny(&value.resource_arn) .Extract(),
 ExpectedRevisionId: crate::standard_library_conversions::ostring_to_dafny(&value.expected_revision_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyInput,
    >,
) -> aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyInput {
    aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyInput::builder()
        .set_resource_arn(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.ResourceArn(),
            ),
        ))
        .set_expected_revision_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ExpectedRevisionId().clone(),
        ))
        .build()
        .unwrap()
}
