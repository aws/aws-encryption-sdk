// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput::DeleteCacheEntryInput {
        identifier: crate::standard_library_conversions::blob_to_dafny(&value.identifier.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput{
    crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput::builder()
        .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
        .build()
        .unwrap()
}
