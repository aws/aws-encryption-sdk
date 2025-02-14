// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::WrappingKeySpec,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::WrappingKeySpec::Rsa2048 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_2048 {},
aws_sdk_kms::types::WrappingKeySpec::Rsa3072 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_3072 {},
aws_sdk_kms::types::WrappingKeySpec::Rsa4096 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_4096 {},
aws_sdk_kms::types::WrappingKeySpec::Sm2 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::SM2 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec,
) -> aws_sdk_kms::types::WrappingKeySpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_2048 {} => aws_sdk_kms::types::WrappingKeySpec::Rsa2048,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_3072 {} => aws_sdk_kms::types::WrappingKeySpec::Rsa3072,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::RSA_4096 {} => aws_sdk_kms::types::WrappingKeySpec::Rsa4096,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::WrappingKeySpec::SM2 {} => aws_sdk_kms::types::WrappingKeySpec::Sm2,
    }
}
