// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsRsaKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsRsaKeyringInput::CreateAwsKmsRsaKeyringInput {
        publicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 kmsKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.kms_key_id) .Extract(),
 encryptionAlgorithm: crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(value.encryption_algorithm.clone().unwrap()),
 kmsClient: ::dafny_runtime::Rc::new(match &value.kms_client {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::client::to_dafny(&x.clone()) },
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsRsaKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyringInput::builder()
        .set_public_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.publicKey().clone()))
 .set_kms_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.kmsKeyId()) ))
 .set_encryption_algorithm(Some( crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(dafny_value.encryptionAlgorithm()) ))
 .set_kms_client(match (*dafny_value.kmsClient()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::client::from_dafny(value.clone())),
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
