// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::connect_custom_key_store::ConnectCustomKeyStoreInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectCustomKeyStoreRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectCustomKeyStoreRequest::ConnectCustomKeyStoreRequest {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectCustomKeyStoreRequest,
    >,
) -> aws_sdk_kms::operation::connect_custom_key_store::ConnectCustomKeyStoreInput {
    aws_sdk_kms::operation::connect_custom_key_store::ConnectCustomKeyStoreInput::builder()
        .set_custom_key_store_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.CustomKeyStoreId(),
            ),
        ))
        .build()
        .unwrap()
}
