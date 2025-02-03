// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeKinesisStreamingDestinationOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeKinesisStreamingDestinationOutput::DescribeKinesisStreamingDestinationOutput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 KinesisDataStreamDestinations: ::dafny_runtime::Rc::new(match &value.kinesis_data_stream_destinations {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::kinesis_data_stream_destination::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeKinesisStreamingDestinationOutput,
    >
) -> aws_sdk_dynamodb::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput{
    aws_sdk_dynamodb::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_kinesis_data_stream_destinations(match (*dafny_value.KinesisDataStreamDestinations()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KinesisDataStreamDestination>| crate::deps::com_amazonaws_dynamodb::conversions::kinesis_data_stream_destination::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
