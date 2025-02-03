// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_contributor_insights::UpdateContributorInsightsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContributorInsightsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContributorInsightsOutput::UpdateContributorInsightsOutput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 ContributorInsightsStatus: ::dafny_runtime::Rc::new(match &value.contributor_insights_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateContributorInsightsOutput,
    >,
) -> aws_sdk_dynamodb::operation::update_contributor_insights::UpdateContributorInsightsOutput {
    aws_sdk_dynamodb::operation::update_contributor_insights::UpdateContributorInsightsOutput::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_contributor_insights_status(match &**dafny_value.ContributorInsightsStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_status::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
}
