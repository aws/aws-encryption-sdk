// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ProvisionedThroughput,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput::ProvisionedThroughput {
        ReadCapacityUnits: value.read_capacity_units,
 WriteCapacityUnits: value.write_capacity_units,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput,
    >,
) -> aws_sdk_dynamodb::types::ProvisionedThroughput {
    aws_sdk_dynamodb::types::ProvisionedThroughput::builder()
        .set_read_capacity_units(Some(dafny_value.ReadCapacityUnits().clone()))
        .set_write_capacity_units(Some(dafny_value.WriteCapacityUnits().clone()))
        .build()
        .unwrap()
}
