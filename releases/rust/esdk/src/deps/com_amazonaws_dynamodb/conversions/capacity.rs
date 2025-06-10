// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::Capacity,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity::Capacity {
        ReadCapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.read_capacity_units),
 WriteCapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.write_capacity_units),
 CapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.capacity_units),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity,
    >,
) -> aws_sdk_dynamodb::types::Capacity {
    aws_sdk_dynamodb::types::Capacity::builder()
        .set_read_capacity_units(crate::standard_library_conversions::odouble_from_dafny(
            dafny_value.ReadCapacityUnits().clone(),
        ))
        .set_write_capacity_units(crate::standard_library_conversions::odouble_from_dafny(
            dafny_value.WriteCapacityUnits().clone(),
        ))
        .set_capacity_units(crate::standard_library_conversions::odouble_from_dafny(
            dafny_value.CapacityUnits().clone(),
        ))
        .build()
}
