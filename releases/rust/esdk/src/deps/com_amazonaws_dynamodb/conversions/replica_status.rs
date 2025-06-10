// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ReplicaStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ReplicaStatus::Creating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::CREATING {},
aws_sdk_dynamodb::types::ReplicaStatus::CreationFailed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::CREATION_FAILED {},
aws_sdk_dynamodb::types::ReplicaStatus::Updating => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::UPDATING {},
aws_sdk_dynamodb::types::ReplicaStatus::Deleting => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::DELETING {},
aws_sdk_dynamodb::types::ReplicaStatus::Active => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::ACTIVE {},
aws_sdk_dynamodb::types::ReplicaStatus::RegionDisabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::REGION_DISABLED {},
aws_sdk_dynamodb::types::ReplicaStatus::InaccessibleEncryptionCredentials => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus,
) -> aws_sdk_dynamodb::types::ReplicaStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::CREATING {} => aws_sdk_dynamodb::types::ReplicaStatus::Creating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::CREATION_FAILED {} => aws_sdk_dynamodb::types::ReplicaStatus::CreationFailed,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::UPDATING {} => aws_sdk_dynamodb::types::ReplicaStatus::Updating,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::DELETING {} => aws_sdk_dynamodb::types::ReplicaStatus::Deleting,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::ACTIVE {} => aws_sdk_dynamodb::types::ReplicaStatus::Active,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::REGION_DISABLED {} => aws_sdk_dynamodb::types::ReplicaStatus::RegionDisabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => aws_sdk_dynamodb::types::ReplicaStatus::InaccessibleEncryptionCredentials,
    }
}
