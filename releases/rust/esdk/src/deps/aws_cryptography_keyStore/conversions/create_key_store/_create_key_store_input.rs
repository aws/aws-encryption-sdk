// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput::CreateKeyStoreInput {

    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput {
    crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput::builder()

        .build()
        .unwrap()
}
