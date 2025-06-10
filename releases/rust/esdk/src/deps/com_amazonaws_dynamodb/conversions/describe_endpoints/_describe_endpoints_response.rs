// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsResponse::DescribeEndpointsResponse {
        Endpoints: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.endpoints,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::endpoint::to_dafny(e)
,
)
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsResponse,
    >,
) -> aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsOutput {
    aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsOutput::builder()
          .set_endpoints(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.Endpoints(),
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Endpoint>| crate::deps::com_amazonaws_dynamodb::conversions::endpoint::from_dafny(e.clone())
,
)
 ))
          .build()
          .unwrap()
}
