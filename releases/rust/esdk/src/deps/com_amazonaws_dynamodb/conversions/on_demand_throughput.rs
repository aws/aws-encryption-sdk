// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::OnDemandThroughput,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::OnDemandThroughput>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::OnDemandThroughput::OnDemandThroughput {
        MaxReadRequestUnits: crate::standard_library_conversions::olong_to_dafny(&value.max_read_request_units),
 MaxWriteRequestUnits: crate::standard_library_conversions::olong_to_dafny(&value.max_write_request_units),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::OnDemandThroughput,
    >,
) -> aws_sdk_dynamodb::types::OnDemandThroughput {
    aws_sdk_dynamodb::types::OnDemandThroughput::builder()
        .set_max_read_request_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.MaxReadRequestUnits().clone(),
        ))
        .set_max_write_request_units(crate::standard_library_conversions::olong_from_dafny(
            dafny_value.MaxWriteRequestUnits().clone(),
        ))
        .build()
}
