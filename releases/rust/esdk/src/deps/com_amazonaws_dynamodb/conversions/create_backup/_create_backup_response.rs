// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::create_backup::CreateBackupOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateBackupOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateBackupOutput::CreateBackupOutput {
        BackupDetails: ::dafny_runtime::Rc::new(match &value.backup_details {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_details::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateBackupOutput,
    >,
) -> aws_sdk_dynamodb::operation::create_backup::CreateBackupOutput {
    aws_sdk_dynamodb::operation::create_backup::CreateBackupOutput::builder()
        .set_backup_details(match (*dafny_value.BackupDetails()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::backup_details::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .build()
}
