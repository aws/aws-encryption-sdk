// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_kms::operation::create_key::CreateKeyError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    match value {
      aws_sdk_kms::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_kms::operation::create_key::CreateKeyError::CloudHsmClusterInvalidConfigurationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_invalid_configuration_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::CustomKeyStoreInvalidStateException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_invalid_state_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::CustomKeyStoreNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_not_found_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::DependencyTimeoutException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::dependency_timeout_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::InvalidArnException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::invalid_arn_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::KmsInternalException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::kms_internal_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::LimitExceededException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::limit_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::MalformedPolicyDocumentException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::malformed_policy_document_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::TagException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::tag_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::UnsupportedOperationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::unsupported_operation_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::XksKeyAlreadyInUseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_key_already_in_use_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::XksKeyInvalidConfigurationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_key_invalid_configuration_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::create_key::CreateKeyError::XksKeyNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_key_not_found_exception::to_dafny(e.clone()),
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

pub mod _create_key_request;

pub mod _create_key_response;
