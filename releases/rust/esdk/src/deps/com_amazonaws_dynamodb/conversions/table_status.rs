// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::TableStatus,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::TableStatus::Creating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::CREATING {},
aws_sdk_dynamodb::types::TableStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::UPDATING {},
aws_sdk_dynamodb::types::TableStatus::Deleting => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::DELETING {},
aws_sdk_dynamodb::types::TableStatus::Active => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ACTIVE {},
aws_sdk_dynamodb::types::TableStatus::InaccessibleEncryptionCredentials => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {},
aws_sdk_dynamodb::types::TableStatus::Archiving => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ARCHIVING {},
aws_sdk_dynamodb::types::TableStatus::Archived => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ARCHIVED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus,
) -> aws_sdk_dynamodb::types::TableStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::CREATING {} => aws_sdk_dynamodb::types::TableStatus::Creating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::UPDATING {} => aws_sdk_dynamodb::types::TableStatus::Updating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::DELETING {} => aws_sdk_dynamodb::types::TableStatus::Deleting,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ACTIVE {} => aws_sdk_dynamodb::types::TableStatus::Active,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => aws_sdk_dynamodb::types::TableStatus::InaccessibleEncryptionCredentials,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ARCHIVING {} => aws_sdk_dynamodb::types::TableStatus::Archiving,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus::ARCHIVED {} => aws_sdk_dynamodb::types::TableStatus::Archived,
    }
}
