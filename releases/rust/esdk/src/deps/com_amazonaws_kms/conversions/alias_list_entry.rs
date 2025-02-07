// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::AliasListEntry,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AliasListEntry,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AliasListEntry::AliasListEntry {
        AliasName: crate::standard_library_conversions::ostring_to_dafny(&value.alias_name),
 AliasArn: crate::standard_library_conversions::ostring_to_dafny(&value.alias_arn),
 TargetKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.target_key_id),
 CreationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date),
 LastUpdatedDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_updated_date),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::AliasListEntry,
    >,
) -> aws_sdk_kms::types::AliasListEntry {
    aws_sdk_kms::types::AliasListEntry::builder()
        .set_alias_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.AliasName().clone(),
        ))
        .set_alias_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.AliasArn().clone(),
        ))
        .set_target_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TargetKeyId().clone(),
        ))
        .set_creation_date(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.CreationDate().clone(),
        ))
        .set_last_updated_date(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.LastUpdatedDate().clone(),
        ))
        .build()
}
