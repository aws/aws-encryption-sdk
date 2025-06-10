// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::KeyManagerType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::KeyManagerType::Aws => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType::AWS {},
aws_sdk_kms::types::KeyManagerType::Customer => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType::CUSTOMER {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType,
) -> aws_sdk_kms::types::KeyManagerType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType::AWS {} => aws_sdk_kms::types::KeyManagerType::Aws,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyManagerType::CUSTOMER {} => aws_sdk_kms::types::KeyManagerType::Customer,
    }
}
