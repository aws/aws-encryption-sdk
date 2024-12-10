// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyOutput::DeleteResourcePolicyOutput {
        RevisionId: crate::standard_library_conversions::ostring_to_dafny(&value.revision_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteResourcePolicyOutput,
    >
) -> aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyOutput {
    aws_sdk_dynamodb::operation::delete_resource_policy::DeleteResourcePolicyOutput::builder()
          .set_revision_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RevisionId().clone()))
          .build()


}
