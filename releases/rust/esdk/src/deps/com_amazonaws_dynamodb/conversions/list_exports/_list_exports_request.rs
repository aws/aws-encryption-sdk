// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_exports::ListExportsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListExportsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListExportsInput::ListExportsInput {
        TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 MaxResults: crate::standard_library_conversions::oint_to_dafny(value.max_results),
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListExportsInput,
    >,
) -> aws_sdk_dynamodb::operation::list_exports::ListExportsInput {
    aws_sdk_dynamodb::operation::list_exports::ListExportsInput::builder()
        .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableArn().clone(),
        ))
        .set_max_results(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.MaxResults().clone(),
        ))
        .set_next_token(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.NextToken().clone(),
        ))
        .build()
        .unwrap()
}
