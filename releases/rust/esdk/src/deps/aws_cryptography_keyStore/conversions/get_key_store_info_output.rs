// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput
{
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput::GetKeyStoreInfoOutput {
        keyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_id) .Extract(),
 keyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.key_store_name) .Extract(),
 logicalKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.logical_key_store_name) .Extract(),
 grantTokens: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.grant_tokens.clone().unwrap(),
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
)
,
 kmsConfiguration: crate::deps::aws_cryptography_keyStore::conversions::kms_configuration::to_dafny(&value.kms_configuration.clone().unwrap())
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
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
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
) -> crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput::GetKeyStoreInfoOutput {..} =>
            crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput::builder()
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
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
