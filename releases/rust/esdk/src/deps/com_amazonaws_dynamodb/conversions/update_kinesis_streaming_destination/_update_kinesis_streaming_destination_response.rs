// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateKinesisStreamingDestinationOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateKinesisStreamingDestinationOutput::UpdateKinesisStreamingDestinationOutput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 StreamArn: crate::standard_library_conversions::ostring_to_dafny(&value.stream_arn),
 DestinationStatus: ::dafny_runtime::Rc::new(match &value.destination_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::destination_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 UpdateKinesisStreamingConfiguration: ::dafny_runtime::Rc::new(match &value.update_kinesis_streaming_configuration {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::update_kinesis_streaming_configuration::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateKinesisStreamingDestinationOutput,
    >
) -> aws_sdk_dynamodb::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput{
    aws_sdk_dynamodb::operation::update_kinesis_streaming_destination::UpdateKinesisStreamingDestinationOutput::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_stream_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.StreamArn().clone()))
 .set_destination_status(match &**dafny_value.DestinationStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::destination_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_update_kinesis_streaming_configuration(match (*dafny_value.UpdateKinesisStreamingConfiguration()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::update_kinesis_streaming_configuration::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
