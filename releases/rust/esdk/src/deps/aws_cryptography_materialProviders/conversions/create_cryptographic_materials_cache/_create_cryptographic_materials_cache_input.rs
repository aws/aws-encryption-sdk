// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput::CreateCryptographicMaterialsCacheInput {
        cache: crate::deps::aws_cryptography_materialProviders::conversions::cache_type::to_dafny(&value.cache.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateCryptographicMaterialsCacheInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput::builder()
        .set_cache(Some( crate::deps::aws_cryptography_materialProviders::conversions::cache_type::from_dafny(dafny_value.cache().clone())
 ))
        .build()
        .unwrap()
}
