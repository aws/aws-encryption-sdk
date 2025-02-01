// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_backup::DescribeBackupOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeBackupOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeBackupOutput::DescribeBackupOutput {
        BackupDescription: ::dafny_runtime::Rc::new(match &value.backup_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::backup_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeBackupOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_backup::DescribeBackupOutput {
    aws_sdk_dynamodb::operation::describe_backup::DescribeBackupOutput::builder()
        .set_backup_description(match (*dafny_value.BackupDescription()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::backup_description::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .build()
}
