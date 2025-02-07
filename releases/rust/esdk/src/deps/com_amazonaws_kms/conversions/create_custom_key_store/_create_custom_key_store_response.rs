// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreResponse::CreateCustomKeyStoreResponse {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreResponse,
    >,
) -> aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreOutput {
    aws_sdk_kms::operation::create_custom_key_store::CreateCustomKeyStoreOutput::builder()
        .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.CustomKeyStoreId().clone(),
        ))
        .build()
}
