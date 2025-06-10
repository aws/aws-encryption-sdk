// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput::CreateKeyInput {
        branchKeyIdentifier: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_identifier),
 encryptionContext:
::dafny_runtime::Rc::new(match &value.encryption_context {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b),
            |v| dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyInput {
    crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyInput::builder()
        .set_branch_key_identifier(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.branchKeyIdentifier().clone(),
        ))
        .set_encryption_context(match (*dafny_value.encryptionContext()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(
                    value,
                    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| {
                        ::std::string::String::from_utf8(
                            dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(
                                &::std::borrow::Borrow::borrow(k),
                                |b| *b,
                            ),
                        )
                        .unwrap()
                    },
                    |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| {
                        ::std::string::String::from_utf8(
                            dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(
                                &::std::borrow::Borrow::borrow(v),
                                |b| *b,
                            ),
                        )
                        .unwrap()
                    },
                ),
            ),
            _ => None,
        })
        .build()
        .unwrap()
}
