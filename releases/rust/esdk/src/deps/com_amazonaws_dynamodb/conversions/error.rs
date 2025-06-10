// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod backup_in_use_exception;

pub mod backup_not_found_exception;

pub mod conditional_check_failed_exception;

pub mod continuous_backups_unavailable_exception;

pub mod duplicate_item_exception;

pub mod export_conflict_exception;

pub mod export_not_found_exception;

pub mod global_table_already_exists_exception;

pub mod global_table_not_found_exception;

pub mod idempotent_parameter_mismatch_exception;

pub mod import_conflict_exception;

pub mod import_not_found_exception;

pub mod index_not_found_exception;

pub mod internal_server_error;

pub mod invalid_endpoint_exception;

pub mod invalid_export_time_exception;

pub mod invalid_restore_time_exception;

pub mod item_collection_size_limit_exceeded_exception;

pub mod limit_exceeded_exception;

pub mod point_in_time_recovery_unavailable_exception;

pub mod policy_not_found_exception;

pub mod provisioned_throughput_exceeded_exception;

pub mod replica_already_exists_exception;

pub mod replica_not_found_exception;

pub mod request_limit_exceeded;

pub mod resource_in_use_exception;

pub mod resource_not_found_exception;

pub mod table_already_exists_exception;

pub mod table_in_use_exception;

pub mod table_not_found_exception;

pub mod transaction_canceled_exception;

pub mod transaction_conflict_exception;

pub mod transaction_in_progress_exception;
/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error(
    value: String,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    let error_msg = value.clone();
    let error_msg =
        ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
            &error_msg,
        );
    let error_obj: ::dafny_runtime::Object<::dafny_runtime::DynAny> =
        ::dafny_runtime::Object(Some(::dafny_runtime::Rc::new(
            ::dafny_runtime::UnsafeCell::new(value),
        )));
    ::dafny_runtime::Rc::new(
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::OpaqueWithText {
            obj: error_obj,
	    objMessage: error_msg
        },
    )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: ::dafny_runtime::DafnyType>(value: String) ->
    ::dafny_runtime::Rc<
        crate::_Wrappers_Compile::Result<
            T,
            ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
        >
    >
{
    ::dafny_runtime::Rc::new(crate::_Wrappers_Compile::Result::Failure {
        error: to_opaque_error(value),
    })
}
pub fn to_dafny(
    value: crate::deps::com_amazonaws_dynamodb::types::error::Error,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
        crate::deps::com_amazonaws_dynamodb::types::error::Error::BackupInUseException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::backup_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::BackupNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::backup_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ConditionalCheckFailedException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::conditional_check_failed_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ContinuousBackupsUnavailableException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::continuous_backups_unavailable_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::DuplicateItemException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::duplicate_item_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ExportConflictException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::export_conflict_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ExportNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::export_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::GlobalTableAlreadyExistsException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::global_table_already_exists_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::GlobalTableNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::global_table_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::IdempotentParameterMismatchException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::idempotent_parameter_mismatch_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ImportConflictException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::import_conflict_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ImportNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::import_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::IndexNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::index_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::InternalServerError { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::internal_server_error::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidEndpointException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_endpoint_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidExportTimeException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_export_time_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidRestoreTimeException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::invalid_restore_time_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ItemCollectionSizeLimitExceededException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::item_collection_size_limit_exceeded_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::LimitExceededException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::limit_exceeded_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::PointInTimeRecoveryUnavailableException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::point_in_time_recovery_unavailable_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::PolicyNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::policy_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ProvisionedThroughputExceededException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::provisioned_throughput_exceeded_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ReplicaAlreadyExistsException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::replica_already_exists_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ReplicaNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::replica_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::RequestLimitExceeded { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::request_limit_exceeded::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ResourceInUseException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::resource_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::ResourceNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::resource_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TableAlreadyExistsException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::table_already_exists_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TableInUseException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::table_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TableNotFoundException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::table_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionCanceledException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::transaction_canceled_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionConflictException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::transaction_conflict_exception::to_dafny(error),
crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionInProgressException { error } =>
    crate::deps::com_amazonaws_dynamodb::conversions::error::transaction_in_progress_exception::to_dafny(error),
        crate::deps::com_amazonaws_dynamodb::types::error::Error::Opaque { obj } =>
            ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::Opaque {
                obj: ::dafny_runtime::Object(obj.0)
            }),
        crate::deps::com_amazonaws_dynamodb::types::error::Error::OpaqueWithText { obj, objMessage } =>
            ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object(obj.0),
                objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&objMessage),
            }),
    }
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
    >,
) -> crate::deps::com_amazonaws_dynamodb::types::error::Error {
    match ::std::borrow::Borrow::borrow(&dafny_value) {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::BackupInUseException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::BackupInUseException {
    error: aws_sdk_dynamodb::types::error::BackupInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::BackupNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::BackupNotFoundException {
    error: aws_sdk_dynamodb::types::error::BackupNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ConditionalCheckFailedException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ConditionalCheckFailedException {
    error: aws_sdk_dynamodb::types::error::ConditionalCheckFailedException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ContinuousBackupsUnavailableException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ContinuousBackupsUnavailableException {
    error: aws_sdk_dynamodb::types::error::ContinuousBackupsUnavailableException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::DuplicateItemException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::DuplicateItemException {
    error: aws_sdk_dynamodb::types::error::DuplicateItemException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ExportConflictException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ExportConflictException {
    error: aws_sdk_dynamodb::types::error::ExportConflictException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ExportNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ExportNotFoundException {
    error: aws_sdk_dynamodb::types::error::ExportNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::GlobalTableAlreadyExistsException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::GlobalTableAlreadyExistsException {
    error: aws_sdk_dynamodb::types::error::GlobalTableAlreadyExistsException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::GlobalTableNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::GlobalTableNotFoundException {
    error: aws_sdk_dynamodb::types::error::GlobalTableNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::IdempotentParameterMismatchException { Message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::IdempotentParameterMismatchException {
    error: aws_sdk_dynamodb::types::error::IdempotentParameterMismatchException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(Message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ImportConflictException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ImportConflictException {
    error: aws_sdk_dynamodb::types::error::ImportConflictException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ImportNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ImportNotFoundException {
    error: aws_sdk_dynamodb::types::error::ImportNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::IndexNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::IndexNotFoundException {
    error: aws_sdk_dynamodb::types::error::IndexNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::InternalServerError { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::InternalServerError {
    error: aws_sdk_dynamodb::types::error::InternalServerError::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::InvalidEndpointException { Message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidEndpointException {
    error: aws_sdk_dynamodb::types::error::InvalidEndpointException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(Message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::InvalidExportTimeException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidExportTimeException {
    error: aws_sdk_dynamodb::types::error::InvalidExportTimeException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::InvalidRestoreTimeException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::InvalidRestoreTimeException {
    error: aws_sdk_dynamodb::types::error::InvalidRestoreTimeException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ItemCollectionSizeLimitExceededException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ItemCollectionSizeLimitExceededException {
    error: aws_sdk_dynamodb::types::error::ItemCollectionSizeLimitExceededException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::LimitExceededException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::LimitExceededException {
    error: aws_sdk_dynamodb::types::error::LimitExceededException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::PointInTimeRecoveryUnavailableException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::PointInTimeRecoveryUnavailableException {
    error: aws_sdk_dynamodb::types::error::PointInTimeRecoveryUnavailableException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::PolicyNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::PolicyNotFoundException {
    error: aws_sdk_dynamodb::types::error::PolicyNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ProvisionedThroughputExceededException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ProvisionedThroughputExceededException {
    error: aws_sdk_dynamodb::types::error::ProvisionedThroughputExceededException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ReplicaAlreadyExistsException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ReplicaAlreadyExistsException {
    error: aws_sdk_dynamodb::types::error::ReplicaAlreadyExistsException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ReplicaNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ReplicaNotFoundException {
    error: aws_sdk_dynamodb::types::error::ReplicaNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::RequestLimitExceeded { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::RequestLimitExceeded {
    error: aws_sdk_dynamodb::types::error::RequestLimitExceeded::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ResourceInUseException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ResourceInUseException {
    error: aws_sdk_dynamodb::types::error::ResourceInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::ResourceNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::ResourceNotFoundException {
    error: aws_sdk_dynamodb::types::error::ResourceNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TableAlreadyExistsException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TableAlreadyExistsException {
    error: aws_sdk_dynamodb::types::error::TableAlreadyExistsException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TableInUseException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TableInUseException {
    error: aws_sdk_dynamodb::types::error::TableInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TableNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TableNotFoundException {
    error: aws_sdk_dynamodb::types::error::TableNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TransactionCanceledException { Message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionCanceledException {
    error: aws_sdk_dynamodb::types::error::TransactionCanceledException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(Message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TransactionConflictException { message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionConflictException {
    error: aws_sdk_dynamodb::types::error::TransactionConflictException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TransactionInProgressException { Message, .. } =>
  crate::deps::com_amazonaws_dynamodb::types::error::Error::TransactionInProgressException {
    error: aws_sdk_dynamodb::types::error::TransactionInProgressException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(Message.clone()))
      .build()
  },
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::Opaque { obj } =>
            crate::deps::com_amazonaws_dynamodb::types::error::Error::Opaque {
                obj: obj.clone()
            },
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::OpaqueWithText { obj, objMessage } =>
            crate::deps::com_amazonaws_dynamodb::types::error::Error::OpaqueWithText {
                obj: obj.clone(),
                objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&objMessage),
            },
    }
}
