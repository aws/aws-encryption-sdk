// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::KinesisDataStreamDestination,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KinesisDataStreamDestination>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KinesisDataStreamDestination::KinesisDataStreamDestination {
        StreamArn: crate::standard_library_conversions::ostring_to_dafny(&value.stream_arn),
 DestinationStatus: ::dafny_runtime::Rc::new(match &value.destination_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::destination_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DestinationStatusDescription: crate::standard_library_conversions::ostring_to_dafny(&value.destination_status_description),
 ApproximateCreationDateTimePrecision: ::dafny_runtime::Rc::new(match &value.approximate_creation_date_time_precision {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::approximate_creation_date_time_precision::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KinesisDataStreamDestination,
    >,
) -> aws_sdk_dynamodb::types::KinesisDataStreamDestination {
    aws_sdk_dynamodb::types::KinesisDataStreamDestination::builder()
          .set_stream_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.StreamArn().clone()))
 .set_destination_status(match &**dafny_value.DestinationStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::destination_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_destination_status_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.DestinationStatusDescription().clone()))
 .set_approximate_creation_date_time_precision(match &**dafny_value.ApproximateCreationDateTimePrecision() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::approximate_creation_date_time_precision::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
}
