// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdOutput::GetBranchKeyIdOutput {
        branchKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_id) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetBranchKeyIdOutput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput{
    crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput::builder()
        .set_branch_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyId()) ))
        .build()
        .unwrap()
}
