// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ExportType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ExportType::FullExport => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType::FULL_EXPORT {},
aws_sdk_dynamodb::types::ExportType::IncrementalExport => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType::INCREMENTAL_EXPORT {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType,
) -> aws_sdk_dynamodb::types::ExportType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType::FULL_EXPORT {} => aws_sdk_dynamodb::types::ExportType::FullExport,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportType::INCREMENTAL_EXPORT {} => aws_sdk_dynamodb::types::ExportType::IncrementalExport,
    }
}
