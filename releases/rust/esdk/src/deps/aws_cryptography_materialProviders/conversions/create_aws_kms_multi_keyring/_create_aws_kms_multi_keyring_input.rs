// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::CreateAwsKmsMultiKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsMultiKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsMultiKeyringInput::CreateAwsKmsMultiKeyringInput {
        generator: crate::standard_library_conversions::ostring_to_dafny(&value.generator),
 kmsKeyIds: ::dafny_runtime::Rc::new(match &value.kms_key_ids {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 clientSupplier: ::dafny_runtime::Rc::new(match &value.client_supplier {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::client_supplier::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 grantTokens: ::dafny_runtime::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsMultiKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::CreateAwsKmsMultiKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::CreateAwsKmsMultiKeyringInput::builder()
        .set_generator(crate::standard_library_conversions::ostring_from_dafny(dafny_value.generator().clone()))
 .set_kms_key_ids(match (*dafny_value.kmsKeyIds()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_client_supplier(match (*dafny_value.clientSupplier()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::client_supplier::from_dafny(value.clone())),
    _ => None,
}
)
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
        .build()
        .unwrap()
}
