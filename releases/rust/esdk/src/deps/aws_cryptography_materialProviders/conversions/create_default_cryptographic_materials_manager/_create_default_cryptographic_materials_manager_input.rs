// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultCryptographicMaterialsManagerInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultCryptographicMaterialsManagerInput::CreateDefaultCryptographicMaterialsManagerInput {
        keyring: crate::deps::aws_cryptography_materialProviders::conversions::keyring::to_dafny(&value.keyring.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultCryptographicMaterialsManagerInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerInput::builder()
        .set_keyring(Some( crate::deps::aws_cryptography_materialProviders::conversions::keyring::from_dafny(dafny_value.keyring().clone())
 ))
        .build()
        .unwrap()
}
