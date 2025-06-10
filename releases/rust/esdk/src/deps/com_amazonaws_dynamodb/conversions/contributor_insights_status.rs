// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ContributorInsightsStatus,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::ENABLING {},
aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::ENABLED {},
aws_sdk_dynamodb::types::ContributorInsightsStatus::Disabling => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::DISABLING {},
aws_sdk_dynamodb::types::ContributorInsightsStatus::Disabled => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::DISABLED {},
aws_sdk_dynamodb::types::ContributorInsightsStatus::Failed => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::FAILED {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus,
) -> aws_sdk_dynamodb::types::ContributorInsightsStatus {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::ENABLING {} => aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::ENABLED {} => aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::DISABLING {} => aws_sdk_dynamodb::types::ContributorInsightsStatus::Disabling,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::DISABLED {} => aws_sdk_dynamodb::types::ContributorInsightsStatus::Disabled,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsStatus::FAILED {} => aws_sdk_dynamodb::types::ContributorInsightsStatus::Failed,
    }
}
