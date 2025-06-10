// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::DeleteGlobalSecondaryIndexAction,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteGlobalSecondaryIndexAction>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteGlobalSecondaryIndexAction::DeleteGlobalSecondaryIndexAction {
        IndexName: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.index_name),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteGlobalSecondaryIndexAction,
    >,
) -> aws_sdk_dynamodb::types::DeleteGlobalSecondaryIndexAction {
    aws_sdk_dynamodb::types::DeleteGlobalSecondaryIndexAction::builder()
        .set_index_name(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.IndexName(),
            ),
        ))
        .build()
        .unwrap()
}
