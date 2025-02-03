// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::RotationsListEntry,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationsListEntry>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationsListEntry::RotationsListEntry {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 RotationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.rotation_date),
 RotationType: ::dafny_runtime::Rc::new(match &value.rotation_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::rotation_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationsListEntry,
    >,
) -> aws_sdk_kms::types::RotationsListEntry {
    aws_sdk_kms::types::RotationsListEntry::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_rotation_date(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.RotationDate().clone(),
        ))
        .set_rotation_type(match &**dafny_value.RotationType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => {
                Some(crate::deps::com_amazonaws_kms::conversions::rotation_type::from_dafny(value))
            }
            _ => None,
        })
        .build()
}
