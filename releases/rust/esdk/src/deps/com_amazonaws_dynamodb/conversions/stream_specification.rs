// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::StreamSpecification,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification::StreamSpecification {
        StreamEnabled: value.stream_enabled.clone(),
 StreamViewType: ::dafny_runtime::Rc::new(match &value.stream_view_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::stream_view_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification,
    >,
) -> aws_sdk_dynamodb::types::StreamSpecification {
    aws_sdk_dynamodb::types::StreamSpecification::builder()
        .set_stream_enabled(Some(dafny_value.StreamEnabled().clone()))
        .set_stream_view_type(match &**dafny_value.StreamViewType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::stream_view_type::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
        .unwrap()
}
