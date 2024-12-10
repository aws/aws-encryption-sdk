// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::import_table::ImportTableOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableOutput::ImportTableOutput {
        ImportTableDescription: crate::deps::com_amazonaws_dynamodb::conversions::import_table_description::to_dafny(&value.import_table_description.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableOutput,
    >
) -> aws_sdk_dynamodb::operation::import_table::ImportTableOutput {
    aws_sdk_dynamodb::operation::import_table::ImportTableOutput::builder()
          .set_import_table_description(Some( crate::deps::com_amazonaws_dynamodb::conversions::import_table_description::from_dafny(dafny_value.ImportTableDescription().clone())
 ))
          .build()


}
