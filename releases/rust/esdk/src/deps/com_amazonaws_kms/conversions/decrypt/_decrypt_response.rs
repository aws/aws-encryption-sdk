// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::decrypt::DecryptOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptResponse,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptResponse::DecryptResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 Plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext),
 EncryptionAlgorithm: ::dafny_runtime::Rc::new(match &value.encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptResponse,
    >,
) -> aws_sdk_kms::operation::decrypt::DecryptOutput {
    aws_sdk_kms::operation::decrypt::DecryptOutput::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_plaintext(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.Plaintext().clone(),
        ))
        .set_encryption_algorithm(match &**dafny_value.EncryptionAlgorithm() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.CiphertextForRecipient().clone(),
        ))
        .build()
}
