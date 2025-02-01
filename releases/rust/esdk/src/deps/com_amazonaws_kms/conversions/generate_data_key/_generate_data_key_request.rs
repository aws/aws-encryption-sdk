// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_data_key::GenerateDataKeyInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyRequest::GenerateDataKeyRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 EncryptionContext:
::dafny_runtime::Rc::new(match &value.encryption_context {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&v),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 NumberOfBytes: crate::standard_library_conversions::oint_to_dafny(value.number_of_bytes),
 KeySpec: ::dafny_runtime::Rc::new(match &value.key_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::data_key_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 GrantTokens: ::dafny_runtime::Rc::new(match &value.grant_tokens {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 Recipient: ::dafny_runtime::Rc::new(match &value.recipient {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::recipient_info::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DryRun: crate::standard_library_conversions::obool_to_dafny(&value.dry_run),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyRequest,
    >,
) -> aws_sdk_kms::operation::generate_data_key::GenerateDataKeyInput {
    aws_sdk_kms::operation::generate_data_key::GenerateDataKeyInput::builder()
          .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_encryption_context(match (*dafny_value.EncryptionContext()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(v),
            )
        ),
    _ => None
}
)
 .set_number_of_bytes(crate::standard_library_conversions::oint_from_dafny(dafny_value.NumberOfBytes().clone()))
 .set_key_spec(match &**dafny_value.KeySpec() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::data_key_spec::from_dafny(value)
    ),
    _ => None,
}
)
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
 .set_recipient(match (*dafny_value.Recipient()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::recipient_info::from_dafny(value.clone())),
    _ => None,
}
)
 .set_dry_run(crate::standard_library_conversions::obool_from_dafny(dafny_value.DryRun().clone()))
          .build()
          .unwrap()
}
