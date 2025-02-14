// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::describe_key::DescribeKeyOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeKeyResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeKeyResponse::DescribeKeyResponse {
        KeyMetadata: ::dafny_runtime::Rc::new(match &value.key_metadata {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_metadata::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeKeyResponse,
    >,
) -> aws_sdk_kms::operation::describe_key::DescribeKeyOutput {
    aws_sdk_kms::operation::describe_key::DescribeKeyOutput::builder()
        .set_key_metadata(match (*dafny_value.KeyMetadata()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::key_metadata::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .build()
}
