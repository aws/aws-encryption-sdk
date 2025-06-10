// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::OnDecryptInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::OnDecryptInput::OnDecryptInput {
        materials: crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::to_dafny(&value.materials.clone().unwrap())
,
 encryptedDataKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_keys.clone().unwrap(),
    |e| crate::deps::aws_cryptography_materialProviders::conversions::encrypted_data_key::to_dafny(&e.clone())
,
)
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::OnDecryptInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput {
    crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput::builder()
        .set_materials(Some( crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_encrypted_data_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedDataKeys(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EncryptedDataKey>| crate::deps::aws_cryptography_materialProviders::conversions::encrypted_data_key::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
