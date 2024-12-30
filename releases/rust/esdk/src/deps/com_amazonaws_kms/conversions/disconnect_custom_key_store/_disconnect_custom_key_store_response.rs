// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisconnectCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisconnectCustomKeyStoreResponse::DisconnectCustomKeyStoreResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisconnectCustomKeyStoreResponse,
    >
) -> aws_sdk_kms::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreOutput {
    aws_sdk_kms::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreOutput::builder()

          .build()


}
