// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum Error {
    AlreadyExistsException {
        error: aws_sdk_kms::types::error::AlreadyExistsException,
    },

    CloudHsmClusterInUseException {
        error: aws_sdk_kms::types::error::CloudHsmClusterInUseException,
    },

    CloudHsmClusterInvalidConfigurationException {
        error: aws_sdk_kms::types::error::CloudHsmClusterInvalidConfigurationException,
    },

    CloudHsmClusterNotActiveException {
        error: aws_sdk_kms::types::error::CloudHsmClusterNotActiveException,
    },

    CloudHsmClusterNotFoundException {
        error: aws_sdk_kms::types::error::CloudHsmClusterNotFoundException,
    },

    CloudHsmClusterNotRelatedException {
        error: aws_sdk_kms::types::error::CloudHsmClusterNotRelatedException,
    },

    ConflictException {
        error: aws_sdk_kms::types::error::ConflictException,
    },

    CustomKeyStoreHasCmKsException {
        error: aws_sdk_kms::types::error::CustomKeyStoreHasCmKsException,
    },

    CustomKeyStoreInvalidStateException {
        error: aws_sdk_kms::types::error::CustomKeyStoreInvalidStateException,
    },

    CustomKeyStoreNameInUseException {
        error: aws_sdk_kms::types::error::CustomKeyStoreNameInUseException,
    },

    CustomKeyStoreNotFoundException {
        error: aws_sdk_kms::types::error::CustomKeyStoreNotFoundException,
    },

    DependencyTimeoutException {
        error: aws_sdk_kms::types::error::DependencyTimeoutException,
    },

    DisabledException {
        error: aws_sdk_kms::types::error::DisabledException,
    },

    DryRunOperationException {
        error: aws_sdk_kms::types::error::DryRunOperationException,
    },

    ExpiredImportTokenException {
        error: aws_sdk_kms::types::error::ExpiredImportTokenException,
    },

    IncorrectKeyException {
        error: aws_sdk_kms::types::error::IncorrectKeyException,
    },

    IncorrectKeyMaterialException {
        error: aws_sdk_kms::types::error::IncorrectKeyMaterialException,
    },

    IncorrectTrustAnchorException {
        error: aws_sdk_kms::types::error::IncorrectTrustAnchorException,
    },

    InvalidAliasNameException {
        error: aws_sdk_kms::types::error::InvalidAliasNameException,
    },

    InvalidArnException {
        error: aws_sdk_kms::types::error::InvalidArnException,
    },

    InvalidCiphertextException {
        error: aws_sdk_kms::types::error::InvalidCiphertextException,
    },

    InvalidGrantIdException {
        error: aws_sdk_kms::types::error::InvalidGrantIdException,
    },

    InvalidGrantTokenException {
        error: aws_sdk_kms::types::error::InvalidGrantTokenException,
    },

    InvalidImportTokenException {
        error: aws_sdk_kms::types::error::InvalidImportTokenException,
    },

    InvalidKeyUsageException {
        error: aws_sdk_kms::types::error::InvalidKeyUsageException,
    },

    InvalidMarkerException {
        error: aws_sdk_kms::types::error::InvalidMarkerException,
    },

    KeyUnavailableException {
        error: aws_sdk_kms::types::error::KeyUnavailableException,
    },

    KmsInternalException {
        error: aws_sdk_kms::types::error::KmsInternalException,
    },

    KmsInvalidMacException {
        error: aws_sdk_kms::types::error::KmsInvalidMacException,
    },

    KmsInvalidSignatureException {
        error: aws_sdk_kms::types::error::KmsInvalidSignatureException,
    },

    KmsInvalidStateException {
        error: aws_sdk_kms::types::error::KmsInvalidStateException,
    },

    LimitExceededException {
        error: aws_sdk_kms::types::error::LimitExceededException,
    },

    MalformedPolicyDocumentException {
        error: aws_sdk_kms::types::error::MalformedPolicyDocumentException,
    },

    NotFoundException {
        error: aws_sdk_kms::types::error::NotFoundException,
    },

    TagException {
        error: aws_sdk_kms::types::error::TagException,
    },

    UnsupportedOperationException {
        error: aws_sdk_kms::types::error::UnsupportedOperationException,
    },

    XksKeyAlreadyInUseException {
        error: aws_sdk_kms::types::error::XksKeyAlreadyInUseException,
    },

    XksKeyInvalidConfigurationException {
        error: aws_sdk_kms::types::error::XksKeyInvalidConfigurationException,
    },

    XksKeyNotFoundException {
        error: aws_sdk_kms::types::error::XksKeyNotFoundException,
    },

    XksProxyIncorrectAuthenticationCredentialException {
        error: aws_sdk_kms::types::error::XksProxyIncorrectAuthenticationCredentialException,
    },

    XksProxyInvalidConfigurationException {
        error: aws_sdk_kms::types::error::XksProxyInvalidConfigurationException,
    },

    XksProxyInvalidResponseException {
        error: aws_sdk_kms::types::error::XksProxyInvalidResponseException,
    },

    XksProxyUriEndpointInUseException {
        error: aws_sdk_kms::types::error::XksProxyUriEndpointInUseException,
    },

    XksProxyUriInUseException {
        error: aws_sdk_kms::types::error::XksProxyUriInUseException,
    },

    XksProxyUriUnreachableException {
        error: aws_sdk_kms::types::error::XksProxyUriUnreachableException,
    },

    XksProxyVpcEndpointServiceInUseException {
        error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceInUseException,
    },

    XksProxyVpcEndpointServiceInvalidConfigurationException {
        error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceInvalidConfigurationException,
    },

    XksProxyVpcEndpointServiceNotFoundException {
        error: aws_sdk_kms::types::error::XksProxyVpcEndpointServiceNotFoundException,
    },
    Opaque {
        obj: ::dafny_runtime::Object<::dafny_runtime::DynAny>,
    },
    OpaqueWithText {
        obj: ::dafny_runtime::Object<::dafny_runtime::DynAny>,
        objMessage: ::std::string::String,
    },
}

impl ::std::cmp::Eq for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for Error {}
