// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::RestoreSummary,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreSummary>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreSummary::RestoreSummary {
        SourceBackupArn: crate::standard_library_conversions::ostring_to_dafny(&value.source_backup_arn),
 SourceTableArn: crate::standard_library_conversions::ostring_to_dafny(&value.source_table_arn),
 RestoreDateTime: crate::standard_library_conversions::timestamp_to_dafny(&value.restore_date_time),
 RestoreInProgress: value.restore_in_progress.clone(),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreSummary,
    >,
) -> aws_sdk_dynamodb::types::RestoreSummary {
    aws_sdk_dynamodb::types::RestoreSummary::builder()
        .set_source_backup_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.SourceBackupArn().clone(),
        ))
        .set_source_table_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.SourceTableArn().clone(),
        ))
        .set_restore_date_time(Some(
            crate::standard_library_conversions::timestamp_from_dafny(
                dafny_value.RestoreDateTime().clone(),
            ),
        ))
        .set_restore_in_progress(Some(dafny_value.RestoreInProgress().clone()))
        .build()
        .unwrap()
}
