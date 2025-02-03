// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::encrypt::EncryptOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EncryptResponse,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EncryptResponse::EncryptResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 EncryptionAlgorithm: ::dafny_runtime::Rc::new(match &value.encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EncryptResponse,
    >,
) -> aws_sdk_kms::operation::encrypt::EncryptOutput {
    aws_sdk_kms::operation::encrypt::EncryptOutput::builder()
        .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.CiphertextBlob().clone(),
        ))
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_encryption_algorithm(match &**dafny_value.EncryptionAlgorithm() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
}
