// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsRequest,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsRequest::DescribeEndpointsRequest {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeEndpointsRequest,
    >
) -> aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsInput {
    aws_sdk_dynamodb::operation::describe_endpoints::DescribeEndpointsInput::builder()

          .build()
          .unwrap()
}
