// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::get_resource_policy::GetResourcePolicyOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetResourcePolicyOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetResourcePolicyOutput::GetResourcePolicyOutput {
        Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy),
 RevisionId: crate::standard_library_conversions::ostring_to_dafny(&value.revision_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GetResourcePolicyOutput,
    >
) -> aws_sdk_dynamodb::operation::get_resource_policy::GetResourcePolicyOutput {
    aws_sdk_dynamodb::operation::get_resource_policy::GetResourcePolicyOutput::builder()
          .set_policy(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Policy().clone()))
 .set_revision_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.RevisionId().clone()))
          .build()


}
