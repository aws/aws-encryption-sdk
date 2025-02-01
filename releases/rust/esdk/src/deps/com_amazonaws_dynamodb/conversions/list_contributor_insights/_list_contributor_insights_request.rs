// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsInput::ListContributorInsightsInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
 MaxResults: crate::standard_library_conversions::oint_to_dafny(value.max_results),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsInput,
    >,
) -> aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsInput {
    aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsInput::builder()
        .set_table_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableName().clone(),
        ))
        .set_next_token(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.NextToken().clone(),
        ))
        .set_max_results(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.MaxResults().clone(),
        ))
        .build()
        .unwrap()
}
