// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresRequest::DescribeCustomKeyStoresRequest {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
 CustomKeyStoreName: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_name),
 Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresRequest,
    >,
) -> aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresInput {
    aws_sdk_kms::operation::describe_custom_key_stores::DescribeCustomKeyStoresInput::builder()
        .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.CustomKeyStoreId().clone(),
        ))
        .set_custom_key_store_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.CustomKeyStoreName().clone(),
        ))
        .set_limit(crate::standard_library_conversions::oint_from_dafny(
            dafny_value.Limit().clone(),
        ))
        .set_marker(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Marker().clone(),
        ))
        .build()
        .unwrap()
}
