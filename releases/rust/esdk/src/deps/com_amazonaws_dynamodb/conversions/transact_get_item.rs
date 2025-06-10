// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TransactGetItem,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItem>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItem::TransactGetItem {
        Get: crate::deps::com_amazonaws_dynamodb::conversions::get::to_dafny(&value.get.clone().unwrap())
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItem,
    >,
) -> aws_sdk_dynamodb::types::TransactGetItem {
    aws_sdk_dynamodb::types::TransactGetItem::builder()
        .set_get(Some(
            crate::deps::com_amazonaws_dynamodb::conversions::get::from_dafny(
                dafny_value.Get().clone(),
            ),
        ))
        .build()
}
