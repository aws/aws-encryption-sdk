// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput::GetKeyStoreInfoOutput {
        keyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_id) .Extract(),
 keyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_name) .Extract(),
 logicalKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.logical_key_store_name) .Extract(),
 grantTokens: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.grant_tokens.clone().unwrap(),
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
)
,
 kmsConfiguration: crate::deps::aws_cryptography_keyStore::conversions::kms_configuration::to_dafny(&value.kms_configuration.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput {
    crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput::builder()
        .set_key_store_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyStoreId()) ))
 .set_key_store_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.keyStoreName()) ))
 .set_logical_key_store_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.logicalKeyStoreName()) ))
 .set_grant_tokens(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.grantTokens(),
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
)
 ))
 .set_kms_configuration(Some( crate::deps::aws_cryptography_keyStore::conversions::kms_configuration::from_dafny(dafny_value.kmsConfiguration().clone())
 ))
        .build()
        .unwrap()
}
