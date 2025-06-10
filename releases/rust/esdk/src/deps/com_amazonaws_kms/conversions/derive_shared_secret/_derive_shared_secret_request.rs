// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretRequest::DeriveSharedSecretRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 KeyAgreementAlgorithm: crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::to_dafny(value.key_agreement_algorithm.clone().unwrap()),
 PublicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key).Extract(),
 GrantTokens: ::dafny_runtime::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 DryRun: crate::standard_library_conversions::obool_to_dafny(&value.dry_run),
 Recipient: ::dafny_runtime::Rc::new(match &value.recipient {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::recipient_info::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretRequest,
    >,
) -> aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretInput {
    aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretInput::builder()
          .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_key_agreement_algorithm(Some( crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::from_dafny(dafny_value.KeyAgreementAlgorithm()) ))
 .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.PublicKey().clone())))
 .set_grant_tokens(match (*dafny_value.GrantTokens()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_dry_run(crate::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
 .set_recipient(match (*dafny_value.Recipient()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::recipient_info::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
