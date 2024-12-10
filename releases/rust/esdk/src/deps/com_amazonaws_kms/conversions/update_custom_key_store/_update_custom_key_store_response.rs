// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreResponse::UpdateCustomKeyStoreResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreResponse,
    >
) -> aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreOutput {
    aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreOutput::builder()

          .build()


}
