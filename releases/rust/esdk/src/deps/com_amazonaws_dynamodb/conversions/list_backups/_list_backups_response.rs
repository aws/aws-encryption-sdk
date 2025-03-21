// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_backups::ListBackupsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsOutput::ListBackupsOutput {
        BackupSummaries: ::dafny_runtime::Rc::new(match &value.backup_summaries {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::backup_summary::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 LastEvaluatedBackupArn: crate::standard_library_conversions::ostring_to_dafny(&value.last_evaluated_backup_arn),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListBackupsOutput,
    >,
) -> aws_sdk_dynamodb::operation::list_backups::ListBackupsOutput {
    aws_sdk_dynamodb::operation::list_backups::ListBackupsOutput::builder()
          .set_backup_summaries(match (*dafny_value.BackupSummaries()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BackupSummary>| crate::deps::com_amazonaws_dynamodb::conversions::backup_summary::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_last_evaluated_backup_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.LastEvaluatedBackupArn().clone()))
          .build()
}
