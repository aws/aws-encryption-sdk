// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_backups::ListBackupsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsInput::ListBackupsInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 TimeRangeLowerBound: crate::standard_library_conversions::otimestamp_to_dafny(&value.time_range_lower_bound),
 TimeRangeUpperBound: crate::standard_library_conversions::otimestamp_to_dafny(&value.time_range_upper_bound),
 ExclusiveStartBackupArn: crate::standard_library_conversions::ostring_to_dafny(&value.exclusive_start_backup_arn),
 BackupType: ::dafny_runtime::Rc::new(match &value.backup_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_type_filter::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsInput,
    >,
) -> aws_sdk_dynamodb::operation::list_backups::ListBackupsInput {
    aws_sdk_dynamodb::operation::list_backups::ListBackupsInput::builder()
        .set_table_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableName().clone(),
        ))
        .set_limit(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.Limit().clone(),
        ))
        .set_time_range_lower_bound(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.TimeRangeLowerBound().clone(),
        ))
        .set_time_range_upper_bound(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.TimeRangeUpperBound().clone(),
        ))
        .set_exclusive_start_backup_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ExclusiveStartBackupArn().clone(),
        ))
        .set_backup_type(match &**dafny_value.BackupType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::backup_type_filter::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
        .unwrap()
}
