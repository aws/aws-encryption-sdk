// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_global_tables::ListGlobalTablesInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListGlobalTablesInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListGlobalTablesInput::ListGlobalTablesInput {
        ExclusiveStartGlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.exclusive_start_global_table_name),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 RegionName: crate::standard_library_conversions::ostring_to_dafny(&value.region_name),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListGlobalTablesInput,
    >,
) -> aws_sdk_dynamodb::operation::list_global_tables::ListGlobalTablesInput {
    aws_sdk_dynamodb::operation::list_global_tables::ListGlobalTablesInput::builder()
        .set_exclusive_start_global_table_name(
            crate::standard_library_conversions::ostring_from_dafny(
                dafny_value.ExclusiveStartGlobalTableName().clone(),
            ),
        )
        .set_limit(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.Limit().clone(),
        ))
        .set_region_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.RegionName().clone(),
        ))
        .build()
        .unwrap()
}
