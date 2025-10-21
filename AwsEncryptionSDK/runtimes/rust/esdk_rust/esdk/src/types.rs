// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(single_use_lifetimes)] // because derive_builder

use crate::Error;
/// Types for the `AwsEncryptionSdkConfig`
use aws_mpl_rs::types::EsdkCommitmentPolicy;
use aws_mpl_rs::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_mpl_rs::types::keyring::KeyringRef;
use derive_builder::Builder;

/// Output Stream
pub trait SafeWrite: std::io::Write + Send + Sync {}
impl<T: std::io::Write + Send + Sync> SafeWrite for T {}

/// Input Stream
pub trait SafeRead: std::io::Read + Send + Sync {}
impl<T: std::io::Read + Send + Sync> SafeRead for T {}

/// Key-Value pairs to associate with the encrypted data
pub type EncryptionContext = ::std::collections::HashMap<String, String>;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Output for `Client::encrypt`
pub struct EncryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: aws_mpl_rs::types::EsdkAlgorithmSuiteId,
    /// data to be decrypted
    pub ciphertext: Vec<u8>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Output for `Client::encrypt_stream`
pub struct EncryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: aws_mpl_rs::types::EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Output for `Client::decrypt`
pub struct DecryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: aws_mpl_rs::types::EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// decrypted data
    pub plaintext: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Output for `Client::decrypt_stream`
pub struct DecryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: aws_mpl_rs::types::EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// During Decryption, Allow or Forbid ESDK-NET v4.0.0 Behavior if the ESDK Message Header fails the Header Authentication check.
#[non_exhaustive]
pub enum NetV400RetryPolicy {
    /// Do not retry on failure
    ForbidRetry,
    /// Retry on failure
    AllowRetry,
}

impl ::std::fmt::Display for NetV400RetryPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ForbidRetry => write!(f, "FORBID_RETRY"),
            Self::AllowRetry => write!(f, "ALLOW_RETRY"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Builder)]
#[builder(setter(into, strip_option), default)]
#[builder(build_fn(error = "Error"))]
#[non_exhaustive]
/// Input for `Client` creation
pub struct AwsEncryptionSdkConfig {
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: Option<EsdkCommitmentPolicy>,
    /// default is no limit
    pub max_encrypted_data_keys: Option<usize>,
    /// default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_0_0_retry_policy: Option<NetV400RetryPolicy>,
}

impl AwsEncryptionSdkConfig {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_encrypted_data_keys == Some(0) {
            Err(crate::error::val_err(
                "max_encrypted_data_keys must not be zero",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
#[builder(build_fn(error = "Error"))]
#[non_exhaustive]
/// Input for `Client` creation
pub struct EncryptInput<'a> {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<aws_mpl_rs::types::EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: Option<&'a EncryptionContext>,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: Option<u32>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    #[builder(setter(into = false))]
    /// data to be encrypted
    pub plaintext: &'a [u8],
}

impl EncryptInput<'_> {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.frame_length == Some(0) {
            Err(crate::error::val_err("frame_length must not be zero"))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(crate::error::val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(crate::error::val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
#[builder(build_fn(error = "Error"))]
#[non_exhaustive]
/// Input for `Client::encrypt_stream`
pub struct EncryptStreamInput<'a> {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<aws_mpl_rs::types::EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: Option<&'a EncryptionContext>,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: Option<u32>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    /// The expected size of the input data stream.
    /// This is only important if you cmm or keyring care about such things, which most don't.
    pub data_size: Option<usize>,
}

impl EncryptStreamInput<'_> {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.frame_length == Some(0) {
            Err(crate::error::val_err("frame_length must not be zero"))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(crate::error::val_err(
                "Either keyring or materials_manager must be provided.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(crate::error::val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
#[builder(build_fn(error = "Error"))]
#[non_exhaustive]
/// Input for `Client::decrypt`
pub struct DecryptInput<'a> {
    #[builder(setter(into = false))]
    /// data to be decrypted
    pub ciphertext: &'a [u8],
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: Option<&'a EncryptionContext>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
}
#[derive(Debug, PartialEq, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default)]
#[builder(build_fn(error = "Error"))]
#[non_exhaustive]
/// Input for `Client::decrypt_stream`
pub struct DecryptStreamInput<'a> {
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: Option<&'a EncryptionContext>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    /// If you decrypt a signed payload, most of the data will be written
    /// to the output stream before the signature is verified.
    /// Thus, if verification fails, you are responsible for discarding any data
    /// already received. If you are willing to accept this, set `i_accept_the_danger` to true.
    /// If verification fails, at least one byte will not have been written to the output stream.
    /// If the ciphertext involves only one frame, then no danger exists, and this flag is not needed.
    pub i_accept_the_danger: Option<bool>,
}

impl DecryptInput<'_> {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(crate::error::val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(crate::error::val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

impl DecryptStreamInput<'_> {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(crate::error::val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(crate::error::val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}
