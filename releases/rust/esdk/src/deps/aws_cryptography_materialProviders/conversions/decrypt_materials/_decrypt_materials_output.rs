// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsOutput::DecryptMaterialsOutput {
        decryptionMaterials: crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::to_dafny(&value.decryption_materials.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DecryptMaterialsOutput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput {
    crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput::builder()
        .set_decryption_materials(Some( crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::from_dafny(dafny_value.decryptionMaterials().clone())
 ))
        .build()
        .unwrap()
}
