// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::StreamViewType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::StreamViewType::NewImage => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::NEW_IMAGE {},
aws_sdk_dynamodb::types::StreamViewType::OldImage => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::OLD_IMAGE {},
aws_sdk_dynamodb::types::StreamViewType::NewAndOldImages => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::NEW_AND_OLD_IMAGES {},
aws_sdk_dynamodb::types::StreamViewType::KeysOnly => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::KEYS_ONLY {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType,
) -> aws_sdk_dynamodb::types::StreamViewType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::NEW_IMAGE {} => aws_sdk_dynamodb::types::StreamViewType::NewImage,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::OLD_IMAGE {} => aws_sdk_dynamodb::types::StreamViewType::OldImage,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::NEW_AND_OLD_IMAGES {} => aws_sdk_dynamodb::types::StreamViewType::NewAndOldImages,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType::KEYS_ONLY {} => aws_sdk_dynamodb::types::StreamViewType::KeysOnly,
    }
}
