// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::ExpirationModelType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::ExpirationModelType::KeyMaterialExpires => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType::KEY_MATERIAL_EXPIRES {},
aws_sdk_kms::types::ExpirationModelType::KeyMaterialDoesNotExpire => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType::KEY_MATERIAL_DOES_NOT_EXPIRE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType,
) -> aws_sdk_kms::types::ExpirationModelType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType::KEY_MATERIAL_EXPIRES {} => aws_sdk_kms::types::ExpirationModelType::KeyMaterialExpires,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ExpirationModelType::KEY_MATERIAL_DOES_NOT_EXPIRE {} => aws_sdk_kms::types::ExpirationModelType::KeyMaterialDoesNotExpire,
    }
}
