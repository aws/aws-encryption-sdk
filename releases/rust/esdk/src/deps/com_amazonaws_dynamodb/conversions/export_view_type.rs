// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ExportViewType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ExportViewType::NewImage => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType::NEW_IMAGE {},
aws_sdk_dynamodb::types::ExportViewType::NewAndOldImages => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType::NEW_AND_OLD_IMAGES {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType,
) -> aws_sdk_dynamodb::types::ExportViewType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType::NEW_IMAGE {} => aws_sdk_dynamodb::types::ExportViewType::NewImage,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportViewType::NEW_AND_OLD_IMAGES {} => aws_sdk_dynamodb::types::ExportViewType::NewAndOldImages,
    }
}
