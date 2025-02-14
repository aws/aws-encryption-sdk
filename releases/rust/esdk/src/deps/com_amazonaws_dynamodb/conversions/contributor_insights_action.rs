// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_dynamodb::types::ContributorInsightsAction,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::ContributorInsightsAction::Enable => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction::ENABLE {},
aws_sdk_dynamodb::types::ContributorInsightsAction::Disable => crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction::DISABLE {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction,
) -> aws_sdk_dynamodb::types::ContributorInsightsAction {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction::ENABLE {} => aws_sdk_dynamodb::types::ContributorInsightsAction::Enable,
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsAction::DISABLE {} => aws_sdk_dynamodb::types::ContributorInsightsAction::Disable,
    }
}
