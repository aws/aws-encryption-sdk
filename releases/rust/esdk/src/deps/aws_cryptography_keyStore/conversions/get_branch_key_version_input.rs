// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput
{
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput::GetBranchKeyVersionInput {
        branchKeyIdentifier: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_identifier) .Extract(),
 branchKeyVersion: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_version) .Extract(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
>>>{
    ::dafny_runtime::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::dafny_runtime::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
    >,
) -> crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
) -> crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput::GetBranchKeyVersionInput {..} =>
            crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput::builder()
                .set_branch_key_identifier(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyIdentifier()) ))
 .set_branch_key_version(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyVersion()) ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput>
{
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
