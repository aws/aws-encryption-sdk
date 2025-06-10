// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::MessageType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::MessageType::Raw => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType::RAW {},
aws_sdk_kms::types::MessageType::Digest => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType::DIGEST {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType,
) -> aws_sdk_kms::types::MessageType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType::RAW {} => aws_sdk_kms::types::MessageType::Raw,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MessageType::DIGEST {} => aws_sdk_kms::types::MessageType::Digest,
    }
}
