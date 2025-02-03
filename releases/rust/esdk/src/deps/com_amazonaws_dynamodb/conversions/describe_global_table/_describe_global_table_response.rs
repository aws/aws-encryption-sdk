// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableOutput::DescribeGlobalTableOutput {
        GlobalTableDescription: ::dafny_runtime::Rc::new(match &value.global_table_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::global_table_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeGlobalTableOutput,
    >,
) -> aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableOutput {
    aws_sdk_dynamodb::operation::describe_global_table::DescribeGlobalTableOutput::builder()
          .set_global_table_description(match (*dafny_value.GlobalTableDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::global_table_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
