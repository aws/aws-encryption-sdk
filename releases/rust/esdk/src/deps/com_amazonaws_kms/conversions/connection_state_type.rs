// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::ConnectionStateType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::ConnectionStateType::Connected => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::CONNECTED {},
aws_sdk_kms::types::ConnectionStateType::Connecting => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::CONNECTING {},
aws_sdk_kms::types::ConnectionStateType::Failed => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::FAILED {},
aws_sdk_kms::types::ConnectionStateType::Disconnected => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::DISCONNECTED {},
aws_sdk_kms::types::ConnectionStateType::Disconnecting => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::DISCONNECTING {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType,
) -> aws_sdk_kms::types::ConnectionStateType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::CONNECTED {} => aws_sdk_kms::types::ConnectionStateType::Connected,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::CONNECTING {} => aws_sdk_kms::types::ConnectionStateType::Connecting,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::FAILED {} => aws_sdk_kms::types::ConnectionStateType::Failed,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::DISCONNECTED {} => aws_sdk_kms::types::ConnectionStateType::Disconnected,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectionStateType::DISCONNECTING {} => aws_sdk_kms::types::ConnectionStateType::Disconnecting,
    }
}
