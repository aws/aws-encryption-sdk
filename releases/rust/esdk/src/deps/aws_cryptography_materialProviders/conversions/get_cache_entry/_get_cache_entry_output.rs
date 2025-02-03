// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput::GetCacheEntryOutput {
        materials: crate::deps::aws_cryptography_materialProviders::conversions::materials::to_dafny(&value.materials.clone().unwrap())
,
 creationTime: value.creation_time.clone().unwrap(),
 expiryTime: value.expiry_time.clone().unwrap(),
 messagesUsed: value.messages_used.clone().unwrap(),
 bytesUsed: value.bytes_used.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput
{
    crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput::builder()
        .set_materials(Some( crate::deps::aws_cryptography_materialProviders::conversions::materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_creation_time(Some( dafny_value.creationTime() .clone() ))
 .set_expiry_time(Some( dafny_value.expiryTime() .clone() ))
 .set_messages_used(Some( dafny_value.messagesUsed() .clone() ))
 .set_bytes_used(Some( dafny_value.bytesUsed() .clone() ))
        .build()
        .unwrap()
}
