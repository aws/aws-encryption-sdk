// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct AwsEncryptionSdkConfig {
    #[allow(missing_docs)]
pub commitment_policy: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy>,
#[allow(missing_docs)]
pub max_encrypted_data_keys: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)]
pub net_v4_0_0_retry_policy: ::std::option::Option<crate::types::NetV400RetryPolicy>,
}
impl AwsEncryptionSdkConfig {
    #[allow(missing_docs)]
pub fn commitment_policy(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy> {
    &self.commitment_policy
}
#[allow(missing_docs)]
pub fn max_encrypted_data_keys(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.max_encrypted_data_keys
}
#[allow(missing_docs)]
pub fn net_v4_0_0_retry_policy(&self) -> &::std::option::Option<crate::types::NetV400RetryPolicy> {
    &self.net_v4_0_0_retry_policy
}
}
impl AwsEncryptionSdkConfig {
    /// Creates a new builder-style object to manufacture [`AwsEncryptionSdkConfig`](crate::types::AwsEncryptionSdkConfig).
    pub fn builder() -> crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfigBuilder {
        crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfigBuilder::default()
    }
}

/// A builder for [`AwsEncryptionSdkConfig`](crate::types::AwsEncryptionSdkConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsEncryptionSdkConfigBuilder {
    pub(crate) commitment_policy: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy>,
pub(crate) max_encrypted_data_keys: ::std::option::Option<::std::primitive::i64>,
pub(crate) net_v4_0_0_retry_policy: ::std::option::Option<crate::types::NetV400RetryPolicy>,
}
impl AwsEncryptionSdkConfigBuilder {
    #[allow(missing_docs)]
pub fn commitment_policy(mut self, input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy>) -> Self {
    self.commitment_policy = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_commitment_policy(mut self, input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy>) -> Self {
    self.commitment_policy = input;
    self
}
#[allow(missing_docs)]
pub fn get_commitment_policy(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy> {
    &self.commitment_policy
}
#[allow(missing_docs)]
pub fn max_encrypted_data_keys(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.max_encrypted_data_keys = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_max_encrypted_data_keys(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.max_encrypted_data_keys = input;
    self
}
#[allow(missing_docs)]
pub fn get_max_encrypted_data_keys(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.max_encrypted_data_keys
}
#[allow(missing_docs)]
pub fn net_v4_0_0_retry_policy(mut self, input: impl ::std::convert::Into<crate::types::NetV400RetryPolicy>) -> Self {
    self.net_v4_0_0_retry_policy = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_net_v4_0_0_retry_policy(mut self, input: ::std::option::Option<crate::types::NetV400RetryPolicy>) -> Self {
    self.net_v4_0_0_retry_policy = input;
    self
}
#[allow(missing_docs)]
pub fn get_net_v4_0_0_retry_policy(&self) -> &::std::option::Option<crate::types::NetV400RetryPolicy> {
    &self.net_v4_0_0_retry_policy
}
    /// Consumes the builder and constructs a [`AwsEncryptionSdkConfig`](crate::types::AwsEncryptionSdkConfig).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig {
            commitment_policy: self.commitment_policy,
max_encrypted_data_keys: self.max_encrypted_data_keys,
net_v4_0_0_retry_policy: self.net_v4_0_0_retry_policy,
        })
    }
}
