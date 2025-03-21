// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_limits::DescribeLimitsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeLimitsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeLimitsOutput::DescribeLimitsOutput {
        AccountMaxReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.account_max_read_capacity_units),
 AccountMaxWriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.account_max_write_capacity_units),
 TableMaxReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.table_max_read_capacity_units),
 TableMaxWriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.table_max_write_capacity_units),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeLimitsOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_limits::DescribeLimitsOutput {
    aws_sdk_dynamodb::operation::describe_limits::DescribeLimitsOutput::builder()
        .set_account_max_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.AccountMaxReadCapacityUnits().clone(),
        ))
        .set_account_max_write_capacity_units(
            crate::standard_library_conversions::olong_from_dafny(
                dafny_value.AccountMaxWriteCapacityUnits().clone(),
            ),
        )
        .set_table_max_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.TableMaxReadCapacityUnits().clone(),
        ))
        .set_table_max_write_capacity_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.TableMaxWriteCapacityUnits().clone(),
        ))
        .build()
}
