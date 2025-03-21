// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::MultiRegionKeyType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::MultiRegionKeyType::Primary => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType::PRIMARY {},
aws_sdk_kms::types::MultiRegionKeyType::Replica => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType::REPLICA {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType,
) -> aws_sdk_kms::types::MultiRegionKeyType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType::PRIMARY {} => aws_sdk_kms::types::MultiRegionKeyType::Primary,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKeyType::REPLICA {} => aws_sdk_kms::types::MultiRegionKeyType::Replica,
    }
}
