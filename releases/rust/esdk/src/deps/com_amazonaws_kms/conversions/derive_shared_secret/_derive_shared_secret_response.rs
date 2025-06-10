// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretResponse::DeriveSharedSecretResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SharedSecret: crate::standard_library_conversions::oblob_to_dafny(&value.shared_secret),
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
 KeyAgreementAlgorithm: ::dafny_runtime::Rc::new(match &value.key_agreement_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyOrigin: ::dafny_runtime::Rc::new(match &value.key_origin {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::origin_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretResponse,
    >,
) -> aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretOutput {
    aws_sdk_kms::operation::derive_shared_secret::DeriveSharedSecretOutput::builder()
          .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_shared_secret(crate::standard_library_conversions::oblob_from_dafny(dafny_value.SharedSecret().clone()))
 .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextForRecipient().clone()))
 .set_key_agreement_algorithm(match &**dafny_value.KeyAgreementAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_origin(match &**dafny_value.KeyOrigin() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::origin_type::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
}
