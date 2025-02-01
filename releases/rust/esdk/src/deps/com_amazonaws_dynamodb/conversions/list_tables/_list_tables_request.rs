// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_tables::ListTablesInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTablesInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTablesInput::ListTablesInput {
        ExclusiveStartTableName: crate::standard_library_conversions::ostring_to_dafny(&value.exclusive_start_table_name),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListTablesInput,
    >,
) -> aws_sdk_dynamodb::operation::list_tables::ListTablesInput {
    aws_sdk_dynamodb::operation::list_tables::ListTablesInput::builder()
        .set_exclusive_start_table_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ExclusiveStartTableName().clone(),
        ))
        .set_limit(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.Limit().clone(),
        ))
        .build()
        .unwrap()
}
