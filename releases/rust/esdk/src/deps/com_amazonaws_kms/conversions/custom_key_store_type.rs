// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::CustomKeyStoreType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::CustomKeyStoreType::AwsCloudhsm => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType::AWS_CLOUDHSM {},
aws_sdk_kms::types::CustomKeyStoreType::ExternalKeyStore => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType::EXTERNAL_KEY_STORE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType,
) -> aws_sdk_kms::types::CustomKeyStoreType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType::AWS_CLOUDHSM {} => aws_sdk_kms::types::CustomKeyStoreType::AwsCloudhsm,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CustomKeyStoreType::EXTERNAL_KEY_STORE {} => aws_sdk_kms::types::CustomKeyStoreType::ExternalKeyStore,
    }
}
