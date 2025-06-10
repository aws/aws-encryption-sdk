// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::get_item::GetItemError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
      aws_sdk_dynamodb::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_dynamodb::operation::get_item::GetItemError::InternalServerError(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::internal_server_error::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::get_item::GetItemError::InvalidEndpointException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_endpoint_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::get_item::GetItemError::ProvisionedThroughputExceededException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::provisioned_throughput_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::get_item::GetItemError::RequestLimitExceeded(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::request_limit_exceeded::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::get_item::GetItemError::ResourceNotFoundException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::resource_not_found_exception::to_dafny(e.clone()),
        e => {
          let msg = format!("{:?}", e);
          crate::deps::com_amazonaws_dynamodb::conversions::error::to_opaque_error(msg)
        }
      },
      _ => {
        let msg = format!("{:?}", value);
        crate::deps::com_amazonaws_dynamodb::conversions::error::to_opaque_error(msg)
      }
   }
}

pub mod _get_item_request;

pub mod _get_item_response;
