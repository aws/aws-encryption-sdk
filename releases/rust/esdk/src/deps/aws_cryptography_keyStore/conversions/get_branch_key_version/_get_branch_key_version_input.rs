// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput::GetBranchKeyVersionInput {
        branchKeyIdentifier: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_identifier) .Extract(),
 branchKeyVersion: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_version) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionInput{
    crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionInput::builder()
        .set_branch_key_identifier(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyIdentifier()) ))
 .set_branch_key_version(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyVersion()) ))
        .build()
        .unwrap()
}
