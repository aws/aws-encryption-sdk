// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
      aws_sdk_dynamodb::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::InternalServerError(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::internal_server_error::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::LimitExceededException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::limit_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::ResourceInUseException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::resource_in_use_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError::ResourceNotFoundException(e) =>
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

pub mod _update_table_replica_auto_scaling_request;

pub mod _update_table_replica_auto_scaling_response;
