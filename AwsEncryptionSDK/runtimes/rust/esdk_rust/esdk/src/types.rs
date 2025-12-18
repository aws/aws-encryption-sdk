// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
#![allow(single_use_lifetimes)]

use crate::Error;
use crate::val_err;
use aws_mpl_legacy::types::EsdkAlgorithmSuiteId;
use aws_mpl_legacy::types::EsdkCommitmentPolicy;
use aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_mpl_legacy::types::keyring::KeyringRef;
use derivative::Derivative;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The length of one frame, must be non-zero.
pub struct FrameLength(u32);

impl FrameLength {
    /// Creates a new non-zero `FrameLength`
    pub fn new(value: u32) -> Result<Self, Error> {
        if value > 0 { Ok(Self(value)) } else { Err(val_err("Frame length must not be zero.")) }
    }

    /// Gets the inner primitive value.
    #[must_use]
    pub const fn get(&self) -> u32 {
        self.0
    }

    /// Sets the inner primitive value.
    pub fn set(&mut self, value : u32) -> Result<(), Error> {
        if value > 0 {
            self.0 = value;
            Ok(())
        } else {
            Err(val_err("Frame length must not be zero."))
        }
    }
}

impl Default for FrameLength {
    //= compliance/client-apis/encrypt.txt#2.4.6
    //= type=implication
    //# This
    //# value MUST default to 4096 bytes.
    fn default() -> Self {
        Self(4096)
    }
}

/// Convenience function to return a `MaterialProviders` Client.
#[must_use]
pub fn mpl() -> aws_mpl_legacy::Client {
    aws_mpl_legacy::Client::from_conf(
        aws_mpl_legacy::types::MaterialProvidersConfig::builder()
            .build()
            .unwrap(),
    )
    .unwrap()
}

/// Output Stream
pub trait SafeWrite: std::io::Write + Send + Sync + std::fmt::Debug {}
impl<T: std::io::Write + Send + Sync + std::fmt::Debug> SafeWrite for T {}

/// Input Stream
pub trait SafeRead: std::io::Read + Send + Sync + std::fmt::Debug {}
impl<T: std::io::Read + Send + Sync + std::fmt::Debug> SafeRead for T {}

/// Key-Value pairs to associate with the encrypted data
pub type EncryptionContext = ::std::collections::HashMap<String, String>;

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Output for [`encrypt`](crate::encrypt).
pub struct EncryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    #[derivative(Default(
        value = "EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384"
    ))]
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// data to be decrypted
    pub ciphertext: Vec<u8>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}
#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Output for [`encrypt_stream`](crate::encrypt_stream).
pub struct EncryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    #[derivative(Default(
        value = "EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384"
    ))]
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Output for [decrypt](crate::decrypt).
pub struct DecryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    #[derivative(Default(
        value = "EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384"
    ))]
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// decrypted data
    pub plaintext: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Output for [`decrypt_stream`](crate::decrypt_stream).
pub struct DecryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    #[derivative(Default(
        value = "EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384"
    ))]
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
/// During Decryption, Allow or Forbid ESDK-NET v4.0.0 Behavior if the ESDK Message Header fails the Header Authentication check.
#[non_exhaustive]
pub enum NetV400RetryPolicy {
    /// Do not retry on failure
    ForbidRetry,
    /// Retry on failure
    #[default]
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

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Input for [`encrypt`](crate::encrypt).
pub struct EncryptInput<'a> {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: FrameLength,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    /// data to be encrypted
    pub plaintext: &'a [u8],
    /// default is no limit
    pub max_encrypted_data_keys: Option<usize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    #[derivative(Default(
        value = "aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt"
    ))]
    pub commitment_policy: EsdkCommitmentPolicy,
}

impl<'a> EncryptInput<'a> {
    /// Create default `EncryptInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct an `EncryptInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(
        plaintext: &'a [u8],
        ec: EncryptionContext,
        cmm: CryptographicMaterialsManagerRef,
    ) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            materials_manager: Some(cmm),
            ..Default::default()
        }
    }
    /// Construct an `EncryptInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(plaintext: &'a [u8], ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            keyring: Some(keyring),
            ..Default::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_encrypted_data_keys == Some(0) {
            Err(val_err(
                "max_encrypted_data_keys must not be zero",
            ))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Input for [`encrypt_stream`](crate::encrypt_stream).
pub struct EncryptStreamInput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: FrameLength,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    /// The expected size of the input data stream.
    /// This is only important if you cmm or keyring care about such things, which most don't.
    pub data_size: Option<usize>,
    /// default is no limit
    pub max_encrypted_data_keys: Option<usize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    #[derivative(Default(
        value = "aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt"
    ))]
    pub commitment_policy: EsdkCommitmentPolicy,
}

impl EncryptStreamInput {
    /// Create default `EncryptStreamInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct an `EncryptStreamInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(ec: EncryptionContext, cmm: CryptographicMaterialsManagerRef) -> Self {
        Self {
            encryption_context: ec,
            materials_manager: Some(cmm),
            ..Default::default()
        }
    }
    /// Construct an `EncryptStreamInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            encryption_context: ec,
            keyring: Some(keyring),
            ..Default::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_encrypted_data_keys == Some(0) {
            Err(val_err(
                "max_encrypted_data_keys must not be zero",
            ))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(val_err(
                "Either keyring or materials_manager must be provided.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Input for [`decrypt`](crate::decrypt).
pub struct DecryptInput<'a> {
    /// data to be decrypted
    pub ciphertext: &'a [u8],
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub keyring: Option<KeyringRef>,
    /// Exactly one of `keyring` or `materials_manager` must be set
    pub materials_manager: Option<CryptographicMaterialsManagerRef>,
    /// default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// default is no limit
    pub max_encrypted_data_keys: Option<usize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    #[derivative(Default(
        value = "aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt"
    ))]
    pub commitment_policy: EsdkCommitmentPolicy,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Default)]
#[non_exhaustive]
/// Input for [`decrypt_stream`](crate::decrypt_stream).
pub struct DecryptStreamInput {
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
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
    pub i_accept_the_danger: bool,
    /// default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// default is no limit
    pub max_encrypted_data_keys: Option<usize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    #[derivative(Default(
        value = "aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt"
    ))]
    pub commitment_policy: EsdkCommitmentPolicy,
}

impl<'a> DecryptInput<'a> {
    /// Create default `DecryptInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct a `DecryptInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(
        ciphertext: &'a [u8],
        ec: EncryptionContext,
        cmm: CryptographicMaterialsManagerRef,
    ) -> Self {
        Self {
            ciphertext,
            encryption_context: ec,
            materials_manager: Some(cmm),
            ..Default::default()
        }
    }
    /// Construct a `DecryptInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(ciphertext: &'a [u8], ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            ciphertext,
            encryption_context: ec,
            keyring: Some(keyring),
            ..Default::default()
        }
    }

    /// Construct a `DecryptInput` from an `EncryptInput`
    #[must_use]
    pub fn from_encrypt(ciphertext: &'a [u8], e: &'a EncryptInput<'_>) -> Self {
        Self {
            ciphertext,
            encryption_context: e.encryption_context.clone(),
            keyring: e.keyring.clone(),
            materials_manager: e.materials_manager.clone(),
            commitment_policy: e.commitment_policy,
            max_encrypted_data_keys: e.max_encrypted_data_keys,
            ..Default::default()
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_encrypted_data_keys == Some(0) {
            Err(val_err(
                "max_encrypted_data_keys must not be zero",
            ))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}

impl DecryptStreamInput {
    /// Create default `DecryptStreamInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct a `DecryptStreamInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(ec: EncryptionContext, cmm: CryptographicMaterialsManagerRef) -> Self {
        Self {
            encryption_context: ec,
            materials_manager: Some(cmm),
            ..Default::default()
        }
    }
    /// Construct a `DecryptStreamInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            encryption_context: ec,
            keyring: Some(keyring),
            ..Default::default()
        }
    }

    /// Construct a `DecryptStreamInput` from an `EncryptStreamInput`
    #[must_use]
    pub fn from_encrypt(e: &EncryptInput<'_>) -> Self {
        Self {
            encryption_context: e.encryption_context.clone(),
            keyring: e.keyring.clone(),
            materials_manager: e.materials_manager.clone(),
            commitment_policy: e.commitment_policy,
            max_encrypted_data_keys: e.max_encrypted_data_keys,
            ..Default::default()
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_encrypted_data_keys == Some(0) {
            Err(val_err(
                "max_encrypted_data_keys must not be zero",
            ))
        } else if self.keyring.is_none() && self.materials_manager.is_none() {
            Err(val_err(
                "Either keyring or materials_manager must be set.",
            ))
        } else if self.keyring.is_some() && self.materials_manager.is_some() {
            Err(val_err(
                "You must not provide both keyring and materials_manager.",
            ))
        } else {
            Ok(())
        }
    }
}
