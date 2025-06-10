// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsDiscoveryKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsDiscoveryKeyringInput::CreateAwsKmsDiscoveryKeyringInput {
        kmsClient: crate::deps::com_amazonaws_kms::conversions::client::to_dafny(&value.kms_client.clone().unwrap())
,
 discoveryFilter: ::dafny_runtime::Rc::new(match &value.discovery_filter {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::discovery_filter::to_dafny(&x.clone()) },
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsDiscoveryKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput::builder()
        .set_kms_client(Some( crate::deps::com_amazonaws_kms::conversions::client::from_dafny(dafny_value.kmsClient().clone())
 ))
 .set_discovery_filter(match (*dafny_value.discoveryFilter()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::discovery_filter::from_dafny(value.clone())),
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
