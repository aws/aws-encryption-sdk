// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::RotationType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType,
> {
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::RotationType::Automatic => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType::AUTOMATIC {},
aws_sdk_kms::types::RotationType::OnDemand => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType::ON_DEMAND {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType,
) -> aws_sdk_kms::types::RotationType {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType::AUTOMATIC {} => aws_sdk_kms::types::RotationType::Automatic,
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotationType::ON_DEMAND {} => aws_sdk_kms::types::RotationType::OnDemand,
    }
}
