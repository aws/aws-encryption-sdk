// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod already_exists_exception;

pub mod cloud_hsm_cluster_in_use_exception;

pub mod cloud_hsm_cluster_invalid_configuration_exception;

pub mod cloud_hsm_cluster_not_active_exception;

pub mod cloud_hsm_cluster_not_found_exception;

pub mod cloud_hsm_cluster_not_related_exception;

pub mod conflict_exception;

pub mod custom_key_store_has_cmks_exception;

pub mod custom_key_store_invalid_state_exception;

pub mod custom_key_store_name_in_use_exception;

pub mod custom_key_store_not_found_exception;

pub mod dependency_timeout_exception;

pub mod disabled_exception;

pub mod dry_run_operation_exception;

pub mod expired_import_token_exception;

pub mod incorrect_key_exception;

pub mod incorrect_key_material_exception;

pub mod incorrect_trust_anchor_exception;

pub mod invalid_alias_name_exception;

pub mod invalid_arn_exception;

pub mod invalid_ciphertext_exception;

pub mod invalid_grant_id_exception;

pub mod invalid_grant_token_exception;

pub mod invalid_import_token_exception;

pub mod invalid_key_usage_exception;

pub mod invalid_marker_exception;

pub mod key_unavailable_exception;

pub mod kms_internal_exception;

pub mod kms_invalid_mac_exception;

pub mod kms_invalid_signature_exception;

pub mod kms_invalid_state_exception;

pub mod limit_exceeded_exception;

pub mod malformed_policy_document_exception;

pub mod not_found_exception;

pub mod tag_exception;

pub mod unsupported_operation_exception;

pub mod xks_key_already_in_use_exception;

pub mod xks_key_invalid_configuration_exception;

pub mod xks_key_not_found_exception;

pub mod xks_proxy_incorrect_authentication_credential_exception;

pub mod xks_proxy_invalid_configuration_exception;

pub mod xks_proxy_invalid_response_exception;

pub mod xks_proxy_uri_endpoint_in_use_exception;

pub mod xks_proxy_uri_in_use_exception;

pub mod xks_proxy_uri_unreachable_exception;

pub mod xks_proxy_vpc_endpoint_service_in_use_exception;

pub mod xks_proxy_vpc_endpoint_service_invalid_configuration_exception;

pub mod xks_proxy_vpc_endpoint_service_not_found_exception;
/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error(
    value: String,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    let error_msg = value.clone();
    let error_msg =
        ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
            &error_msg,
        );
    let error_obj: ::dafny_runtime::Object<::dafny_runtime::DynAny> =
        ::dafny_runtime::Object(Some(::dafny_runtime::Rc::new(
            ::dafny_runtime::UnsafeCell::new(value),
        )));
    ::dafny_runtime::Rc::new(
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::OpaqueWithText {
            obj: error_obj,
	    objMessage: error_msg
        },
    )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: ::dafny_runtime::DafnyType>(
    value: String,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Result<
        T,
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
        >,
    >,
> {
    ::dafny_runtime::Rc::new(crate::_Wrappers_Compile::Result::Failure {
        error: to_opaque_error(value),
    })
}
pub fn to_dafny(
    value: crate::deps::com_amazonaws_kms::types::error::Error,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    match value {
        crate::deps::com_amazonaws_kms::types::error::Error::AlreadyExistsException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::already_exists_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterInvalidConfigurationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_invalid_configuration_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotActiveException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_active_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotFoundException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotRelatedException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::cloud_hsm_cluster_not_related_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::ConflictException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::conflict_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreHasCmKsException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_has_cmks_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreInvalidStateException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_invalid_state_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreNameInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_name_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreNotFoundException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::custom_key_store_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::DependencyTimeoutException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::dependency_timeout_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::DisabledException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::disabled_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::DryRunOperationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::dry_run_operation_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::ExpiredImportTokenException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::expired_import_token_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::IncorrectKeyException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::incorrect_key_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::IncorrectKeyMaterialException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::incorrect_key_material_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::IncorrectTrustAnchorException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::incorrect_trust_anchor_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidAliasNameException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_alias_name_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidArnException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_arn_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidCiphertextException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_ciphertext_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidGrantIdException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_grant_id_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidGrantTokenException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_grant_token_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidImportTokenException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_import_token_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidKeyUsageException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_key_usage_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::InvalidMarkerException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::invalid_marker_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::KeyUnavailableException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::key_unavailable_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::KmsInternalException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::kms_internal_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidMacException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::kms_invalid_mac_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidSignatureException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::kms_invalid_signature_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidStateException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::kms_invalid_state_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::LimitExceededException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::limit_exceeded_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::MalformedPolicyDocumentException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::malformed_policy_document_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::NotFoundException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::TagException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::tag_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::UnsupportedOperationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::unsupported_operation_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksKeyAlreadyInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_key_already_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksKeyInvalidConfigurationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_key_invalid_configuration_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksKeyNotFoundException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_key_not_found_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyIncorrectAuthenticationCredentialException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_incorrect_authentication_credential_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyInvalidConfigurationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_invalid_configuration_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyInvalidResponseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_invalid_response_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriEndpointInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_endpoint_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriUnreachableException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_uri_unreachable_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceInUseException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_in_use_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceInvalidConfigurationException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_invalid_configuration_exception::to_dafny(error),
crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceNotFoundException { error } =>
    crate::deps::com_amazonaws_kms::conversions::error::xks_proxy_vpc_endpoint_service_not_found_exception::to_dafny(error),
        crate::deps::com_amazonaws_kms::types::error::Error::Opaque { obj } =>
            ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::Opaque {
                obj: ::dafny_runtime::Object(obj.0)
            }),
        crate::deps::com_amazonaws_kms::types::error::Error::OpaqueWithText { obj, objMessage } =>
            ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object(obj.0),
                objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&objMessage),
            }),
    }
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
    >,
) -> crate::deps::com_amazonaws_kms::types::error::Error {
    match ::std::borrow::Borrow::borrow(&dafny_value) {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::AlreadyExistsException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::AlreadyExistsException {
    error: aws_sdk_kms::types::error::AlreadyExistsException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CloudHsmClusterInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterInUseException {
    error: aws_sdk_kms::types::error::CloudHsmClusterInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CloudHsmClusterInvalidConfigurationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterInvalidConfigurationException {
    error: aws_sdk_kms::types::error::CloudHsmClusterInvalidConfigurationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CloudHsmClusterNotActiveException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotActiveException {
    error: aws_sdk_kms::types::error::CloudHsmClusterNotActiveException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CloudHsmClusterNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotFoundException {
    error: aws_sdk_kms::types::error::CloudHsmClusterNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CloudHsmClusterNotRelatedException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CloudHsmClusterNotRelatedException {
    error: aws_sdk_kms::types::error::CloudHsmClusterNotRelatedException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::ConflictException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::ConflictException {
    error: aws_sdk_kms::types::error::ConflictException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CustomKeyStoreHasCMKsException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreHasCmKsException {
    error: aws_sdk_kms::types::error::CustomKeyStoreHasCmKsException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CustomKeyStoreInvalidStateException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreInvalidStateException {
    error: aws_sdk_kms::types::error::CustomKeyStoreInvalidStateException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CustomKeyStoreNameInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreNameInUseException {
    error: aws_sdk_kms::types::error::CustomKeyStoreNameInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::CustomKeyStoreNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::CustomKeyStoreNotFoundException {
    error: aws_sdk_kms::types::error::CustomKeyStoreNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::DependencyTimeoutException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::DependencyTimeoutException {
    error: aws_sdk_kms::types::error::DependencyTimeoutException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::DisabledException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::DisabledException {
    error: aws_sdk_kms::types::error::DisabledException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::DryRunOperationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::DryRunOperationException {
    error: aws_sdk_kms::types::error::DryRunOperationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::ExpiredImportTokenException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::ExpiredImportTokenException {
    error: aws_sdk_kms::types::error::ExpiredImportTokenException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::IncorrectKeyException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::IncorrectKeyException {
    error: aws_sdk_kms::types::error::IncorrectKeyException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::IncorrectKeyMaterialException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::IncorrectKeyMaterialException {
    error: aws_sdk_kms::types::error::IncorrectKeyMaterialException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::IncorrectTrustAnchorException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::IncorrectTrustAnchorException {
    error: aws_sdk_kms::types::error::IncorrectTrustAnchorException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidAliasNameException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidAliasNameException {
    error: aws_sdk_kms::types::error::InvalidAliasNameException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidArnException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidArnException {
    error: aws_sdk_kms::types::error::InvalidArnException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidCiphertextException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidCiphertextException {
    error: aws_sdk_kms::types::error::InvalidCiphertextException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidGrantIdException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidGrantIdException {
    error: aws_sdk_kms::types::error::InvalidGrantIdException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidGrantTokenException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidGrantTokenException {
    error: aws_sdk_kms::types::error::InvalidGrantTokenException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidImportTokenException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidImportTokenException {
    error: aws_sdk_kms::types::error::InvalidImportTokenException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidKeyUsageException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidKeyUsageException {
    error: aws_sdk_kms::types::error::InvalidKeyUsageException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::InvalidMarkerException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::InvalidMarkerException {
    error: aws_sdk_kms::types::error::InvalidMarkerException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KeyUnavailableException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::KeyUnavailableException {
    error: aws_sdk_kms::types::error::KeyUnavailableException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KMSInternalException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::KmsInternalException {
    error: aws_sdk_kms::types::error::KmsInternalException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KMSInvalidMacException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidMacException {
    error: aws_sdk_kms::types::error::KmsInvalidMacException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KMSInvalidSignatureException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidSignatureException {
    error: aws_sdk_kms::types::error::KmsInvalidSignatureException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KMSInvalidStateException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::KmsInvalidStateException {
    error: aws_sdk_kms::types::error::KmsInvalidStateException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::LimitExceededException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::LimitExceededException {
    error: aws_sdk_kms::types::error::LimitExceededException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::MalformedPolicyDocumentException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::MalformedPolicyDocumentException {
    error: aws_sdk_kms::types::error::MalformedPolicyDocumentException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::NotFoundException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::NotFoundException {
    error: aws_sdk_kms::types::error::NotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::TagException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::TagException {
    error: aws_sdk_kms::types::error::TagException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::UnsupportedOperationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::UnsupportedOperationException {
    error: aws_sdk_kms::types::error::UnsupportedOperationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksKeyAlreadyInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksKeyAlreadyInUseException {
    error: aws_sdk_kms::types::error::XksKeyAlreadyInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksKeyInvalidConfigurationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksKeyInvalidConfigurationException {
    error: aws_sdk_kms::types::error::XksKeyInvalidConfigurationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksKeyNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksKeyNotFoundException {
    error: aws_sdk_kms::types::error::XksKeyNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyIncorrectAuthenticationCredentialException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyIncorrectAuthenticationCredentialException {
    error: aws_sdk_kms::types::error::XksProxyIncorrectAuthenticationCredentialException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyInvalidConfigurationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyInvalidConfigurationException {
    error: aws_sdk_kms::types::error::XksProxyInvalidConfigurationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyInvalidResponseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyInvalidResponseException {
    error: aws_sdk_kms::types::error::XksProxyInvalidResponseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyUriEndpointInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriEndpointInUseException {
    error: aws_sdk_kms::types::error::XksProxyUriEndpointInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyUriInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriInUseException {
    error: aws_sdk_kms::types::error::XksProxyUriInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyUriUnreachableException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyUriUnreachableException {
    error: aws_sdk_kms::types::error::XksProxyUriUnreachableException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyVpcEndpointServiceInUseException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceInUseException {
    error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceInUseException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyVpcEndpointServiceInvalidConfigurationException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceInvalidConfigurationException {
    error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceInvalidConfigurationException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::XksProxyVpcEndpointServiceNotFoundException { message, .. } =>
  crate::deps::com_amazonaws_kms::types::error::Error::XksProxyVpcEndpointServiceNotFoundException {
    error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceNotFoundException::builder()
      .set_message(crate::standard_library_conversions::ostring_from_dafny(message.clone()))
      .build()
  },
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::Opaque { obj } =>
            crate::deps::com_amazonaws_kms::types::error::Error::Opaque {
                obj: obj.clone()
            },
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::OpaqueWithText { obj, objMessage } =>
            crate::deps::com_amazonaws_kms::types::error::Error::OpaqueWithText {
                obj: obj.clone(),
                objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&objMessage),
            },
    }
}
