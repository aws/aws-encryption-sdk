// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    match value {
      aws_sdk_kms::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfigurationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_invalid_configuration_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CloudHsmClusterNotActiveException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_active_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CloudHsmClusterNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_found_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CloudHsmClusterNotRelatedException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_related_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CustomKeyStoreInvalidStateException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_invalid_state_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CustomKeyStoreNameInUseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_name_in_use_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::CustomKeyStoreNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_not_found_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::KmsInternalException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::kms_internal_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyIncorrectAuthenticationCredentialException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_incorrect_authentication_credential_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyInvalidConfigurationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_invalid_configuration_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyInvalidResponseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_invalid_response_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyUriEndpointInUseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_endpoint_in_use_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyUriInUseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_in_use_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyUriUnreachableException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_unreachable_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceInUseException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_in_use_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceInvalidConfigurationException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_invalid_configuration_exception::to_dafny(e.clone()),
         aws_sdk_kms::operation::update_custom_key_store::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceNotFoundException(e) =>
            crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_not_found_exception::to_dafny(e.clone()),
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

pub mod _update_custom_key_store_request;

pub mod _update_custom_key_store_response;
