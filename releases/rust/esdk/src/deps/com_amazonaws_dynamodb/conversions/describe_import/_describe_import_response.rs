// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_import::DescribeImportOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeImportOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeImportOutput::DescribeImportOutput {
        ImportTableDescription: crate::deps::com_amazonaws_dynamodb::conversions::import_table_description::to_dafny(&value.import_table_description.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeImportOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_import::DescribeImportOutput {
    aws_sdk_dynamodb::operation::describe_import::DescribeImportOutput::builder()
        .set_import_table_description(Some(
            crate::deps::com_amazonaws_dynamodb::conversions::import_table_description::from_dafny(
                dafny_value.ImportTableDescription().clone(),
            ),
        ))
        .build()
}
