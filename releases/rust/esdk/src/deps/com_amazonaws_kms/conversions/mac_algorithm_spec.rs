// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::MacAlgorithmSpec,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::MacAlgorithmSpec::HmacSha224 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_224 {},
aws_sdk_kms::types::MacAlgorithmSpec::HmacSha256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_256 {},
aws_sdk_kms::types::MacAlgorithmSpec::HmacSha384 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_384 {},
aws_sdk_kms::types::MacAlgorithmSpec::HmacSha512 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_512 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec,
) -> aws_sdk_kms::types::MacAlgorithmSpec {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_224 {} => aws_sdk_kms::types::MacAlgorithmSpec::HmacSha224,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_256 {} => aws_sdk_kms::types::MacAlgorithmSpec::HmacSha256,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_384 {} => aws_sdk_kms::types::MacAlgorithmSpec::HmacSha384,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec::HMAC_SHA_512 {} => aws_sdk_kms::types::MacAlgorithmSpec::HmacSha512,
    }
}
