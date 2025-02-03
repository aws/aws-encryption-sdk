// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ProvisionedThroughputDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription::ProvisionedThroughputDescription {
        LastIncreaseDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_increase_date_time),
 LastDecreaseDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_decrease_date_time),
 NumberOfDecreasesToday: crate::standard_library_conversions::olong_to_dafny(&value.number_of_decreases_today),
 ReadCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.read_capacity_units),
 WriteCapacityUnits: crate::standard_library_conversions::olong_to_dafny(&value.write_capacity_units),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription,
    >,
) -> aws_sdk_dynamodb::types::ProvisionedThroughputDescription {
    aws_sdk_dynamodb::types::ProvisionedThroughputDescription::builder()
        .set_last_increase_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.LastIncreaseDateTime().clone(),
        ))
        .set_last_decrease_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.LastDecreaseDateTime().clone(),
        ))
        .set_number_of_decreases_today(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.NumberOfDecreasesToday().clone(),
        ))
        .set_read_capacity_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.ReadCapacityUnits().clone(),
        ))
        .set_write_capacity_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.WriteCapacityUnits().clone(),
        ))
        .build()
}
