// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig,
> {
    ::dafny_runtime::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig,
    >,
) -> crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig {
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig::KeyStoreConfig {
        ddbTableName: crate::standard_library_conversions::ostring_to_dafny(&value.ddb_table_name) .Extract(),
 kmsConfiguration: crate::deps::aws_cryptography_keyStore::conversions::kms_configuration::to_dafny(&value.kms_configuration.clone().unwrap())
,
 logicalKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.logical_key_store_name) .Extract(),
 id: crate::standard_library_conversions::ostring_to_dafny(&value.id),
 grantTokens: ::dafny_runtime::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ddbClient: ::dafny_runtime::Rc::new(match &value.ddb_client {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::client::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 kmsClient: ::dafny_runtime::Rc::new(match &value.kms_client {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::client::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig,
) -> crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig::KeyStoreConfig {..} =>
            crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig::builder()
                .set_ddb_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.ddbTableName()) ))
 .set_kms_configuration(Some( crate::deps::aws_cryptography_keyStore::conversions::kms_configuration::from_dafny(dafny_value.kmsConfiguration().clone())
 ))
 .set_logical_key_store_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.logicalKeyStoreName()) ))
 .set_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.id().clone()))
 .set_grant_tokens(match (*dafny_value.grantTokens()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_ddb_client(match (*dafny_value.ddbClient()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::client::from_dafny(value.clone())),
    _ => None,
}
)
 .set_kms_client(match (*dafny_value.kmsClient()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::client::from_dafny(value.clone())),
    _ => None,
}
)
                .build()
                .unwrap()
    }
}
