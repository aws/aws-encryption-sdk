// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidDecryptionMaterialsTransitionInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidDecryptionMaterialsTransitionInput::ValidDecryptionMaterialsTransitionInput {
        start: crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::to_dafny(&value.start.clone().unwrap())
,
 stop: crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::to_dafny(&value.stop.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ValidDecryptionMaterialsTransitionInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput{
    crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput::builder()
        .set_start(Some( crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::from_dafny(dafny_value.start().clone())
 ))
 .set_stop(Some( crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::from_dafny(dafny_value.stop().clone())
 ))
        .build()
        .unwrap()
}
