// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableInput::DescribeGlobalTableInput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableInput,
    >,
) -> aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableInput {
    aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableInput::builder()
        .set_global_table_name(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.GlobalTableName(),
            ),
        ))
        .build()
        .unwrap()
}
