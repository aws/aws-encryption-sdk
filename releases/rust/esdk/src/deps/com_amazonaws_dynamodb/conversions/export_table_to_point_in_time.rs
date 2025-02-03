// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
      aws_sdk_dynamodb::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::ExportConflictException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::export_conflict_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InternalServerError(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::internal_server_error::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::InvalidExportTimeException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_export_time_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::LimitExceededException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::limit_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::PointInTimeRecoveryUnavailableException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::point_in_time_recovery_unavailable_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeError::TableNotFoundException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::table_not_found_exception::to_dafny(e.clone()),
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

pub mod _export_table_to_point_in_time_request;

pub mod _export_table_to_point_in_time_response;
