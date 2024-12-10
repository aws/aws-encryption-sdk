// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ProvisionedThroughputOverride,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride::ProvisionedThroughputOverride {
        ReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.read_capacity_units),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride,
    >,
) -> aws_sdk_dynamodb::types::ProvisionedThroughputOverride {
    aws_sdk_dynamodb::types::ProvisionedThroughputOverride::builder()
          .set_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(dafny_value.ReadCapacityUnits().clone()))
          .build()

}
