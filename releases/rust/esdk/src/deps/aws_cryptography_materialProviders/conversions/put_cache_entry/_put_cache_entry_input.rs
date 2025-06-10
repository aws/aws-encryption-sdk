// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput::PutCacheEntryInput {
        identifier: crate::standard_library_conversions::blob_to_dafny(&value.identifier.unwrap()),
 materials: crate::deps::aws_cryptography_materialProviders::conversions::materials::to_dafny(&value.materials.clone().unwrap())
,
 creationTime: value.creation_time.clone().unwrap(),
 expiryTime: value.expiry_time.clone().unwrap(),
 messagesUsed: crate::standard_library_conversions::oint_to_dafny(value.messages_used),
 bytesUsed: crate::standard_library_conversions::oint_to_dafny(value.bytes_used),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput
{
    crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput::builder()
        .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
 .set_materials(Some( crate::deps::aws_cryptography_materialProviders::conversions::materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_creation_time(Some( dafny_value.creationTime() .clone() ))
 .set_expiry_time(Some( dafny_value.expiryTime() .clone() ))
 .set_messages_used(crate::standard_library_conversions::oint_from_dafny(dafny_value.messagesUsed().clone()))
 .set_bytes_used(crate::standard_library_conversions::oint_from_dafny(dafny_value.bytesUsed().clone()))
        .build()
        .unwrap()
}
