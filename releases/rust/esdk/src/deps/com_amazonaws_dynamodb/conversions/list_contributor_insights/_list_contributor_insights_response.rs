// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsOutput::ListContributorInsightsOutput {
        ContributorInsightsSummaries: ::dafny_runtime::Rc::new(match &value.contributor_insights_summaries {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_summary::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListContributorInsightsOutput,
    >,
) -> aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsOutput {
    aws_sdk_dynamodb::operation::list_contributor_insights::ListContributorInsightsOutput::builder()
          .set_contributor_insights_summaries(match (*dafny_value.ContributorInsightsSummaries()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ContributorInsightsSummary>| crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_summary::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_next_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NextToken().clone()))
          .build()
}
