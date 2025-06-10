// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_kms::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    match value {
      aws_sdk_kms::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_kms::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreHasCmKsException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_has_cmks_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreInvalidStateException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_invalid_state_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_not_found_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::delete_custom_key_store::DeleteCustomKeyStoreError::KmsInternalException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::kms_internal_exception::to_dafny(e.clone()),
        e => {
          let msg = format!("{:?}", e);
          crate::deps::com_amazonaws_kms::conversions::error::to_opaque_error(msg)
        }
      },
      _ => {
        let msg = format!("{:?}", value);
        crate::deps::com_amazonaws_kms::conversions::error::to_opaque_error(msg)
      }
   }
}

pub mod _delete_custom_key_store_request;

pub mod _delete_custom_key_store_response;
