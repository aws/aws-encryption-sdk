// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_contributor_insights::DescribeContributorInsightsOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContributorInsightsOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContributorInsightsOutput::DescribeContributorInsightsOutput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 ContributorInsightsRuleList: ::dafny_runtime::Rc::new(match &value.contributor_insights_rule_list {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ContributorInsightsStatus: ::dafny_runtime::Rc::new(match &value.contributor_insights_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LastUpdateDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.last_update_date_time),
 FailureException: ::dafny_runtime::Rc::new(match &value.failure_exception {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::failure_exception::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeContributorInsightsOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_contributor_insights::DescribeContributorInsightsOutput {
    aws_sdk_dynamodb::operation::describe_contributor_insights::DescribeContributorInsightsOutput::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_contributor_insights_rule_list(match (*dafny_value.ContributorInsightsRuleList()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
 .set_contributor_insights_status(match &**dafny_value.ContributorInsightsStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::contributor_insights_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_last_update_date_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.LastUpdateDateTime().clone()))
 .set_failure_exception(match (*dafny_value.FailureException()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::failure_exception::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
