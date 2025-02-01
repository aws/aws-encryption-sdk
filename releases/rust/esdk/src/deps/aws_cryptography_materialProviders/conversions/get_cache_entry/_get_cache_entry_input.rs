// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput::GetCacheEntryInput {
        identifier: crate::standard_library_conversions::blob_to_dafny(&value.identifier.unwrap()),
 bytesUsed: crate::standard_library_conversions::olong_to_dafny(&value.bytes_used),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput
{
    crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput::builder()
        .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
 .set_bytes_used(crate::standard_library_conversions::olong_from_dafny(dafny_value.bytesUsed().clone()))
        .build()
        .unwrap()
}
