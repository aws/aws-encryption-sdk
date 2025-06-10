// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyOutput::PutResourcePolicyOutput {
        RevisionId: crate::standard_library_conversions::ostring_to_dafny(&value.revision_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::PutResourcePolicyOutput,
    >,
) -> aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyOutput {
    aws_sdk_dynamodb::operation::put_resource_policy::PutResourcePolicyOutput::builder()
        .set_revision_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.RevisionId().clone(),
        ))
        .build()
}
