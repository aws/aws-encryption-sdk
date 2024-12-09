// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreOutput::CreateKeyStoreOutput {
        tableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput {
    crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput::builder()
        .set_table_arn(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.tableArn()) ))
        .build()
        .unwrap()
}
