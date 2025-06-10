// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BackupDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDescription::BackupDescription {
        BackupDetails: ::dafny_runtime::Rc::new(match &value.backup_details {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_details::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SourceTableDetails: ::dafny_runtime::Rc::new(match &value.source_table_details {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::source_table_details::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SourceTableFeatureDetails: ::dafny_runtime::Rc::new(match &value.source_table_feature_details {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::source_table_feature_details::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDescription,
    >,
) -> aws_sdk_dynamodb::types::BackupDescription {
    aws_sdk_dynamodb::types::BackupDescription::builder()
          .set_backup_details(match (*dafny_value.BackupDetails()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::backup_details::from_dafny(value.clone())),
    _ => None,
}
)
 .set_source_table_details(match (*dafny_value.SourceTableDetails()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::source_table_details::from_dafny(value.clone())),
    _ => None,
}
)
 .set_source_table_feature_details(match (*dafny_value.SourceTableFeatureDetails()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::source_table_feature_details::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
