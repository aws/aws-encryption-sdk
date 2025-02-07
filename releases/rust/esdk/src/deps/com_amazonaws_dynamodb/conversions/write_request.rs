// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::WriteRequest,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::WriteRequest,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::WriteRequest::WriteRequest {
        PutRequest: ::dafny_runtime::Rc::new(match &value.put_request {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::put_request::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DeleteRequest: ::dafny_runtime::Rc::new(match &value.delete_request {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::delete_request::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::WriteRequest,
    >,
) -> aws_sdk_dynamodb::types::WriteRequest {
    aws_sdk_dynamodb::types::WriteRequest::builder()
        .set_put_request(match (*dafny_value.PutRequest()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::put_request::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .set_delete_request(match (*dafny_value.DeleteRequest()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::delete_request::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .build()
}
