// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsEcdhKeyringInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsEcdhKeyringInput::CreateAwsKmsEcdhKeyringInput {
        KeyAgreementScheme: crate::deps::aws_cryptography_materialProviders::conversions::kms_ecdh_static_configurations::to_dafny(&value.key_agreement_scheme.clone().unwrap())
,
 curveSpec: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.curve_spec.clone().unwrap()),
 kmsClient: crate::deps::com_amazonaws_kms::conversions::client::to_dafny(&value.kms_client.clone().unwrap())
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
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsEcdhKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyringInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyringInput::builder()
        .set_key_agreement_scheme(Some( crate::deps::aws_cryptography_materialProviders::conversions::kms_ecdh_static_configurations::from_dafny(dafny_value.KeyAgreementScheme().clone())
 ))
 .set_curve_spec(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.curveSpec()) ))
 .set_kms_client(Some( crate::deps::com_amazonaws_kms::conversions::client::from_dafny(dafny_value.kmsClient().clone())
 ))
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
