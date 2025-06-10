// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::list_imports::ListImportsInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListImportsInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListImportsInput::ListImportsInput {
        TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 PageSize: crate::standard_library_conversions::oint_to_dafny(value.page_size),
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ListImportsInput,
    >,
) -> aws_sdk_dynamodb::operation::list_imports::ListImportsInput {
    aws_sdk_dynamodb::operation::list_imports::ListImportsInput::builder()
        .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableArn().clone(),
        ))
        .set_page_size(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.PageSize().clone(),
        ))
        .set_next_token(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.NextToken().clone(),
        ))
        .build()
        .unwrap()
}
