// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::EnableKinesisStreamingConfiguration,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::EnableKinesisStreamingConfiguration>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::EnableKinesisStreamingConfiguration::EnableKinesisStreamingConfiguration {
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
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::EnableKinesisStreamingConfiguration,
    >,
) -> aws_sdk_dynamodb::types::EnableKinesisStreamingConfiguration {
    aws_sdk_dynamodb::types::EnableKinesisStreamingConfiguration::builder()
          .set_approximate_creation_date_time_precision(match &**dafny_value.ApproximateCreationDateTimePrecision() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::approximate_creation_date_time_precision::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
}
