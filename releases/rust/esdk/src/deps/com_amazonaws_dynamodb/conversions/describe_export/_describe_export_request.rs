// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_export::DescribeExportInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeExportInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeExportInput::DescribeExportInput {
        ExportArn: crate::standard_library_conversions::ostring_to_dafny(&value.export_arn) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeExportInput,
    >,
) -> aws_sdk_dynamodb::operation::describe_export::DescribeExportInput {
    aws_sdk_dynamodb::operation::describe_export::DescribeExportInput::builder()
        .set_export_arn(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.ExportArn(),
            ),
        ))
        .build()
        .unwrap()
}
