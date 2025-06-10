// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BackupDetails,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDetails>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDetails::BackupDetails {
        BackupArn: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.backup_arn),
 BackupName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.backup_name),
 BackupSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.backup_size_bytes),
 BackupStatus: crate::deps::com_amazonaws_dynamodb::conversions::backup_status::to_dafny(value.backup_status.clone()),
 BackupType: crate::deps::com_amazonaws_dynamodb::conversions::backup_type::to_dafny(value.backup_type.clone()),
 BackupCreationDateTime: crate::standard_library_conversions::timestamp_to_dafny(&value.backup_creation_date_time),
 BackupExpiryDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.backup_expiry_date_time),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupDetails,
    >,
) -> aws_sdk_dynamodb::types::BackupDetails {
    aws_sdk_dynamodb::types::BackupDetails::builder()
        .set_backup_arn(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.BackupArn(),
            ),
        ))
        .set_backup_name(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.BackupName(),
            ),
        ))
        .set_backup_size_bytes(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.BackupSizeBytes().clone(),
        ))
        .set_backup_status(Some(
            crate::deps::com_amazonaws_dynamodb::conversions::backup_status::from_dafny(
                dafny_value.BackupStatus(),
            ),
        ))
        .set_backup_type(Some(
            crate::deps::com_amazonaws_dynamodb::conversions::backup_type::from_dafny(
                dafny_value.BackupType(),
            ),
        ))
        .set_backup_creation_date_time(Some(
            crate::standard_library_conversions::timestamp_from_dafny(
                dafny_value.BackupCreationDateTime().clone(),
            ),
        ))
        .set_backup_expiry_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.BackupExpiryDateTime().clone(),
        ))
        .build()
        .unwrap()
}
