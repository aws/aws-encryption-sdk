// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::KeyListEntry,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyListEntry,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyListEntry::KeyListEntry {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 KeyArn: crate::standard_library_conversions::ostring_to_dafny(&value.key_arn),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyListEntry,
    >,
) -> aws_sdk_kms::types::KeyListEntry {
    aws_sdk_kms::types::KeyListEntry::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_key_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyArn().clone(),
        ))
        .build()
}
