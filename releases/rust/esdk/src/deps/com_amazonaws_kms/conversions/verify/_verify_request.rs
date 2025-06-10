// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::verify::VerifyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyRequest,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyRequest::VerifyRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 Message: crate::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
 MessageType: ::dafny_runtime::Rc::new(match &value.message_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::message_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Signature: crate::standard_library_conversions::oblob_to_dafny(&value.signature).Extract(),
 SigningAlgorithm: crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::to_dafny(value.signing_algorithm.clone().unwrap()),
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
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyRequest,
    >,
) -> aws_sdk_kms::operation::verify::VerifyInput {
    aws_sdk_kms::operation::verify::VerifyInput::builder()
          .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.Message().clone())))
 .set_message_type(match &**dafny_value.MessageType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::message_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_signature(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.Signature().clone())))
 .set_signing_algorithm(Some( crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::from_dafny(dafny_value.SigningAlgorithm()) ))
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
          .build()
          .unwrap()
}
