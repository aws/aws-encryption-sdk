// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BackupSummary,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupSummary>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupSummary::BackupSummary {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 TableId: crate::standard_library_conversions::ostring_to_dafny(&value.table_id),
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 BackupArn: crate::standard_library_conversions::ostring_to_dafny(&value.backup_arn),
 BackupName: crate::standard_library_conversions::ostring_to_dafny(&value.backup_name),
 BackupCreationDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.backup_creation_date_time),
 BackupExpiryDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.backup_expiry_date_time),
 BackupStatus: ::dafny_runtime::Rc::new(match &value.backup_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 BackupType: ::dafny_runtime::Rc::new(match &value.backup_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 BackupSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.backup_size_bytes),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupSummary,
    >,
) -> aws_sdk_dynamodb::types::BackupSummary {
    aws_sdk_dynamodb::types::BackupSummary::builder()
        .set_table_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableName().clone(),
        ))
        .set_table_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableId().clone(),
        ))
        .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableArn().clone(),
        ))
        .set_backup_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.BackupArn().clone(),
        ))
        .set_backup_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.BackupName().clone(),
        ))
        .set_backup_creation_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.BackupCreationDateTime().clone(),
        ))
        .set_backup_expiry_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.BackupExpiryDateTime().clone(),
        ))
        .set_backup_status(match &**dafny_value.BackupStatus() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::backup_status::from_dafny(value),
            ),
            _ => None,
        })
        .set_backup_type(match &**dafny_value.BackupType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::backup_type::from_dafny(value),
            ),
            _ => None,
        })
        .set_backup_size_bytes(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.BackupSizeBytes().clone(),
        ))
        .build()
}
