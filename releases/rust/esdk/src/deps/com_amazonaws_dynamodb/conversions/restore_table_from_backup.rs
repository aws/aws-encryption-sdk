// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
      aws_sdk_dynamodb::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::BackupInUseException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::backup_in_use_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::BackupNotFoundException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::backup_not_found_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::InternalServerError(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::internal_server_error::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::InvalidEndpointException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_endpoint_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::LimitExceededException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::limit_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::TableAlreadyExistsException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::table_already_exists_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::restore_table_from_backup::RestoreTableFromBackupError::TableInUseException(e) =>
            crate::deps::com_amazonaws_dynamodb::conversions::error::table_in_use_exception::to_dafny(e.clone()),
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

pub mod _restore_table_from_backup_request;

pub mod _restore_table_from_backup_response;
