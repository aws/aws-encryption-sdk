// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionOutput::GetBranchKeyVersionOutput {
        branchKeyMaterials: crate::deps::aws_cryptography_keyStore::conversions::branch_key_materials::to_dafny(&value.branch_key_materials.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionOutput{
    crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionOutput::builder()
        .set_branch_key_materials(Some( crate::deps::aws_cryptography_keyStore::conversions::branch_key_materials::from_dafny(dafny_value.branchKeyMaterials().clone())
 ))
        .build()
        .unwrap()
}
