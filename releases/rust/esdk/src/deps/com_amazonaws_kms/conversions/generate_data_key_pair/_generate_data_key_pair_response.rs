// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_data_key_pair::GenerateDataKeyPairOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairResponse::GenerateDataKeyPairResponse {
        PrivateKeyCiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.private_key_ciphertext_blob),
 PrivateKeyPlaintext: crate::standard_library_conversions::oblob_to_dafny(&value.private_key_plaintext),
 PublicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 KeyPairSpec: ::dafny_runtime::Rc::new(match &value.key_pair_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::data_key_pair_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairResponse,
    >,
) -> aws_sdk_kms::operation::generate_data_key_pair::GenerateDataKeyPairOutput {
    aws_sdk_kms::operation::generate_data_key_pair::GenerateDataKeyPairOutput::builder()
        .set_private_key_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.PrivateKeyCiphertextBlob().clone(),
        ))
        .set_private_key_plaintext(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.PrivateKeyPlaintext().clone(),
        ))
        .set_public_key(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.PublicKey().clone(),
        ))
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_key_pair_spec(match &**dafny_value.KeyPairSpec() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::data_key_pair_spec::from_dafny(value),
            ),
            _ => None,
        })
        .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.CiphertextForRecipient().clone(),
        ))
        .build()
}
