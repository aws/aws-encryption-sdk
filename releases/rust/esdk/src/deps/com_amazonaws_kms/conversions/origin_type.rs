// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::OriginType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::OriginType::AwsKms => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::AWS_KMS {},
aws_sdk_kms::types::OriginType::External => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::EXTERNAL {},
aws_sdk_kms::types::OriginType::AwsCloudhsm => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::AWS_CLOUDHSM {},
aws_sdk_kms::types::OriginType::ExternalKeyStore => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::EXTERNAL_KEY_STORE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType,
) -> aws_sdk_kms::types::OriginType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::AWS_KMS {} => aws_sdk_kms::types::OriginType::AwsKms,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::EXTERNAL {} => aws_sdk_kms::types::OriginType::External,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::AWS_CLOUDHSM {} => aws_sdk_kms::types::OriginType::AwsCloudhsm,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::OriginType::EXTERNAL_KEY_STORE {} => aws_sdk_kms::types::OriginType::ExternalKeyStore,
    }
}
