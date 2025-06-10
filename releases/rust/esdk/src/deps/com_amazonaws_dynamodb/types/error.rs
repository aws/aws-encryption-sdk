// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum Error {
    BackupInUseException {
        error: aws_sdk_dynamodb::types::error::BackupInUseException,
    },

    BackupNotFoundException {
        error: aws_sdk_dynamodb::types::error::BackupNotFoundException,
    },

    ConditionalCheckFailedException {
        error: aws_sdk_dynamodb::types::error::ConditionalCheckFailedException,
    },

    ContinuousBackupsUnavailableException {
        error: aws_sdk_dynamodb::types::error::ContinuousBackupsUnavailableException,
    },

    DuplicateItemException {
        error: aws_sdk_dynamodb::types::error::DuplicateItemException,
    },

    ExportConflictException {
        error: aws_sdk_dynamodb::types::error::ExportConflictException,
    },

    ExportNotFoundException {
        error: aws_sdk_dynamodb::types::error::ExportNotFoundException,
    },

    GlobalTableAlreadyExistsException {
        error: aws_sdk_dynamodb::types::error::GlobalTableAlreadyExistsException,
    },

    GlobalTableNotFoundException {
        error: aws_sdk_dynamodb::types::error::GlobalTableNotFoundException,
    },

    IdempotentParameterMismatchException {
        error: aws_sdk_dynamodb::types::error::IdempotentParameterMismatchException,
    },

    ImportConflictException {
        error: aws_sdk_dynamodb::types::error::ImportConflictException,
    },

    ImportNotFoundException {
        error: aws_sdk_dynamodb::types::error::ImportNotFoundException,
    },

    IndexNotFoundException {
        error: aws_sdk_dynamodb::types::error::IndexNotFoundException,
    },

    InternalServerError {
        error: aws_sdk_dynamodb::types::error::InternalServerError,
    },

    InvalidEndpointException {
        error: aws_sdk_dynamodb::types::error::InvalidEndpointException,
    },

    InvalidExportTimeException {
        error: aws_sdk_dynamodb::types::error::InvalidExportTimeException,
    },

    InvalidRestoreTimeException {
        error: aws_sdk_dynamodb::types::error::InvalidRestoreTimeException,
    },

    ItemCollectionSizeLimitExceededException {
        error: aws_sdk_dynamodb::types::error::ItemCollectionSizeLimitExceededException,
    },

    LimitExceededException {
        error: aws_sdk_dynamodb::types::error::LimitExceededException,
    },

    PointInTimeRecoveryUnavailableException {
        error: aws_sdk_dynamodb::types::error::PointInTimeRecoveryUnavailableException,
    },

    PolicyNotFoundException {
        error: aws_sdk_dynamodb::types::error::PolicyNotFoundException,
    },

    ProvisionedThroughputExceededException {
        error: aws_sdk_dynamodb::types::error::ProvisionedThroughputExceededException,
    },

    ReplicaAlreadyExistsException {
        error: aws_sdk_dynamodb::types::error::ReplicaAlreadyExistsException,
    },

    ReplicaNotFoundException {
        error: aws_sdk_dynamodb::types::error::ReplicaNotFoundException,
    },

    RequestLimitExceeded {
        error: aws_sdk_dynamodb::types::error::RequestLimitExceeded,
    },

    ResourceInUseException {
        error: aws_sdk_dynamodb::types::error::ResourceInUseException,
    },

    ResourceNotFoundException {
        error: aws_sdk_dynamodb::types::error::ResourceNotFoundException,
    },

    TableAlreadyExistsException {
        error: aws_sdk_dynamodb::types::error::TableAlreadyExistsException,
    },

    TableInUseException {
        error: aws_sdk_dynamodb::types::error::TableInUseException,
    },

    TableNotFoundException {
        error: aws_sdk_dynamodb::types::error::TableNotFoundException,
    },

    TransactionCanceledException {
        error: aws_sdk_dynamodb::types::error::TransactionCanceledException,
    },

    TransactionConflictException {
        error: aws_sdk_dynamodb::types::error::TransactionConflictException,
    },

    TransactionInProgressException {
        error: aws_sdk_dynamodb::types::error::TransactionInProgressException,
    },
    Opaque {
        obj: ::dafny_runtime::Object<::dafny_runtime::DynAny>,
    },
    OpaqueWithText {
        obj: ::dafny_runtime::Object<::dafny_runtime::DynAny>,
        objMessage: ::std::string::String,
    },
}

impl ::std::cmp::Eq for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for Error {}
