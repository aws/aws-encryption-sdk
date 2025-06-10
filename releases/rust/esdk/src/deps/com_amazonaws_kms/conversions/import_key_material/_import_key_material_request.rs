// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::import_key_material::ImportKeyMaterialInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ImportKeyMaterialRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ImportKeyMaterialRequest::ImportKeyMaterialRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 ImportToken: crate::standard_library_conversions::oblob_to_dafny(&value.import_token).Extract(),
 EncryptedKeyMaterial: crate::standard_library_conversions::oblob_to_dafny(&value.encrypted_key_material).Extract(),
 ValidTo: crate::standard_library_conversions::otimestamp_to_dafny(&value.valid_to),
 ExpirationModel: ::dafny_runtime::Rc::new(match &value.expiration_model {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::expiration_model_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ImportKeyMaterialRequest,
    >,
) -> aws_sdk_kms::operation::import_key_material::ImportKeyMaterialInput {
    aws_sdk_kms::operation::import_key_material::ImportKeyMaterialInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_import_token(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.ImportToken().clone(),
        )))
        .set_encrypted_key_material(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.EncryptedKeyMaterial().clone(),
        )))
        .set_valid_to(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.ValidTo().clone(),
        ))
        .set_expiration_model(match &**dafny_value.ExpirationModel() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::expiration_model_type::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
        .unwrap()
}
