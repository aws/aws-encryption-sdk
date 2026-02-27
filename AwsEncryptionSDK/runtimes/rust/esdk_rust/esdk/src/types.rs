// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::Error;
use crate::val_err;
#[cfg(feature = "legacy")]
use aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef as LegacyCMM;
#[cfg(feature = "legacy")]
use aws_mpl_legacy::types::keyring::KeyringRef as LegacyKeyring;
use aws_mpl_rs::CryptographicMaterialsManagerRef;
use aws_mpl_rs::KeyringRef;
use aws_mpl_rs::commitment::EsdkCommitmentPolicy;
use aws_mpl_rs::suites::EsdkAlgorithmSuiteId;
use std::num::NonZeroUsize;

#[expect(dead_code)]
fn comp(x: &KeyringRef, y: &KeyringRef) -> bool {
    std::ptr::addr_eq(std::sync::Arc::as_ptr(x), std::sync::Arc::as_ptr(y))
}

/// Source for Cryptographic Materials
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MaterialSource {
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    /// Legacy CMM, i.e. `aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef`
    LegacyCmm(LegacyCMM),
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    /// Legacy Keyring, i.e. `aws_mpl_legacy::types::keyring::KeyringRef`
    LegacyKeyring(LegacyKeyring),
    /// CMM
    Cmm(CryptographicMaterialsManagerRef),
    /// Keyring
    Keyring(KeyringRef),
}

impl PartialEq for MaterialSource {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cmm(x), Self::Cmm(y)) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(x), std::sync::Arc::as_ptr(y))
            }
            (Self::Keyring(x), Self::Keyring(y)) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(x), std::sync::Arc::as_ptr(y))
            }
            #[cfg(feature = "legacy")]
            (Self::LegacyCmm(x), Self::LegacyCmm(y)) => x == y,
            #[cfg(feature = "legacy")]
            (Self::LegacyKeyring(x), Self::LegacyKeyring(y)) => x == y,
            _ => false,
        }
    }
}
impl Eq for MaterialSource {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The length of one frame, must be non-zero, defaults to 4096.
//= specification/data-format/message-body.md#framed-data
//# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
#[expect(clippy::exhaustive_structs)]
pub struct FrameLength(pub std::num::NonZeroU32);

impl Default for FrameLength {
    //= specification/client-apis/encrypt.md#frame-length
    //# This value MUST default to 4096 bytes.
    fn default() -> Self {
        Self(std::num::NonZeroU32::new(4096).unwrap())
    }
}

impl FrameLength {
    /// return new `FrameLength`.
    pub fn new(val: u32) -> Result<Self, Error> {
        //= specification/client-apis/encrypt.md#frame-length
        //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
        Ok(Self(
            std::num::NonZeroU32::new(val)
                .ok_or_else(|| val_err("Frame length must be non-zero"))?,
        ))
    }
    /// return new `FrameLength`. Panic if val is zero.
    #[must_use]
    pub const fn new_unchecked(val: u32) -> Self {
        Self(std::num::NonZeroU32::new(val).unwrap())
    }
}

/// Convenience function to return a `MaterialProviders` Client.
#[cfg(feature = "legacy")]
#[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
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
pub use aws_mpl_rs::EncryptionContext;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Output for [`encrypt`](crate::encrypt).
pub struct EncryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// data to be decrypted
    pub ciphertext: Vec<u8>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}
impl EncryptOutput {
    /// Create default `EncryptOutput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Output for [`encrypt_stream`](crate::encrypt_stream).
pub struct EncryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}
impl EncryptStreamOutput {
    /// Create default `EncryptStreamOutput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Output for [decrypt](crate::decrypt).
pub struct DecryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// decrypted data
    pub plaintext: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Output for [`decrypt_stream`](crate::decrypt_stream).
pub struct DecryptStreamOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Input for [`encrypt`](crate::encrypt).
pub struct EncryptInput<'a> {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: FrameLength,
    /// The source of cryptographic materials
    pub source: Option<MaterialSource>,
    /// data to be encrypted
    pub plaintext: &'a [u8],
    //= specification/client-apis/client.md#initialization
    //# On client initialization,
    //# The caller MUST have the option to provide a:
    //# - [commitment policy](#commitment-policy)
    //# - [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys)
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: EsdkCommitmentPolicy,
    /// default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
}

#[allow(
    single_use_lifetimes,
    reason = "Remove when we add with_cmm and with_keyring"
)]
impl<'a> EncryptInput<'a> {
    /// Create default `EncryptInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct an `EncryptInput` with a legacy `CryptographicMaterialsManagerRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_cmm(plaintext: &'a [u8], ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptInput` with a legacy `KeyringRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_keyring(
        plaintext: &'a [u8],
        ec: EncryptionContext,
        keyring: LegacyKeyring,
    ) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            source: Some(MaterialSource::LegacyKeyring(keyring)),
            ..Default::default()
        }
    }
    #[must_use]
    /// Construct an `EncryptInput` with a `CryptographicMaterialsManagerRef`
    pub fn with_cmm(
        plaintext: &'a [u8],
        ec: EncryptionContext,
        cmm: CryptographicMaterialsManagerRef,
    ) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            source: Some(MaterialSource::Cmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(plaintext: &'a [u8], ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            plaintext,
            encryption_context: ec,
            source: Some(MaterialSource::Keyring(keyring)),
            ..Default::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        //= specification/client-apis/encrypt.md#input
        //# The following inputs to this behavior are REQUIRED:
        if self.source.is_none() {
            Err(val_err("A Materials Source must be provided."))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Input for [`encrypt_stream`](crate::encrypt_stream).
pub struct EncryptStreamInput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    pub frame_length: FrameLength,
    /// The source of cryptographic materials
    pub source: Option<MaterialSource>,
    /// The expected size of the input data stream.
    /// This is only important if you cmm or keyring care about such things, which most don't.
    pub data_size: Option<usize>,
    /// default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: EsdkCommitmentPolicy,
}

impl EncryptStreamInput {
    /// Create default `EncryptStreamInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct an `EncryptStreamInput` with a legacy `CryptographicMaterialsManagerRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_cmm(ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptStreamInput` with a legacy `KeyringRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_keyring(ec: EncryptionContext, keyring: LegacyKeyring) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyKeyring(keyring)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptStreamInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(ec: EncryptionContext, cmm: CryptographicMaterialsManagerRef) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::Cmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptStreamInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::Keyring(keyring)),
            ..Default::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        //= specification/client-apis/decrypt.md#input
        //# The client MUST require exactly one of the following types of inputs:
        if self.source.is_none() {
            Err(val_err("A Materials Source must be provided."))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
#[allow(
    single_use_lifetimes,
    reason = "Remove when we add with_cmm and with_keyring"
)]
/// Input for [`decrypt`](crate::decrypt).
pub struct DecryptInput<'a> {
    /// data to be decrypted
    pub ciphertext: &'a [u8],
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// The source of cryptographic materials
    pub source: Option<MaterialSource>,
    /// default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: EsdkCommitmentPolicy,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Input for [`decrypt_stream`](crate::decrypt_stream).
pub struct DecryptStreamInput {
    /// Key-Value pairs to associate with the encrypted data
    pub encryption_context: EncryptionContext,
    /// The source of cryptographic materials
    pub source: Option<MaterialSource>,
    //= specification/client-apis/decrypt.md#authenticated-data
    //= type=exception
    //= reason=Releasing unauthenticated data is behind an "unsafe" feature flag
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    /// If you decrypt a signed payload, most of the data will be written
    /// to the output stream before the signature is verified.
    /// Thus, if verification fails, you are responsible for discarding any data
    /// already received. If you are willing to accept this, set `allow_unsafe_unauthenticated_plaintext_read` to true.
    /// If verification fails, at least one byte will not have been written to the output stream.
    /// If the ciphertext involves only one frame, then no danger exists, and this flag is not needed.
    pub allow_unsafe_unauthenticated_plaintext_read: bool,
    /// default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    pub commitment_policy: EsdkCommitmentPolicy,
}

impl<'a> DecryptInput<'a> {
    /// Create default `DecryptInput`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Construct a `DecryptInput` with a legacy `CryptographicMaterialsManagerRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_cmm(ciphertext: &'a [u8], ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            ciphertext,
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptInput` with a legacy `KeyringRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_keyring(
        ciphertext: &'a [u8],
        ec: EncryptionContext,
        keyring: LegacyKeyring,
    ) -> Self {
        Self {
            ciphertext,
            encryption_context: ec,
            source: Some(MaterialSource::LegacyKeyring(keyring)),
            ..Default::default()
        }
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
            source: Some(MaterialSource::Cmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptInput` with a `KeyringRef`
    #[must_use]
    pub fn with_keyring(ciphertext: &'a [u8], ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            ciphertext,
            encryption_context: ec,
            source: Some(MaterialSource::Keyring(keyring)),
            ..Default::default()
        }
    }

    /// Construct a `DecryptInput` from an `EncryptInput`
    #[must_use]
    pub fn from_encrypt(ciphertext: &'a [u8], e: &'a EncryptInput<'_>) -> Self {
        Self {
            ciphertext,
            encryption_context: e.encryption_context.clone(),
            source: e.source.clone(),
            commitment_policy: e.commitment_policy,
            max_encrypted_data_keys: e.max_encrypted_data_keys,
            ..Default::default()
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        //# The client MUST require the following as inputs to this operation:
        //# - [Encrypted Message](#encrypted-message)
        if self.ciphertext.is_none() {
            Err(val_err("A ciphertext value must be provided."))
        }
        //= specification/client-apis/decrypt.md#input
        //# The client MUST require exactly one of the following types of inputs:
        //# - [Cryptographic Materials Manager (CMM)](../framework/cmm-interface.md)
        //# - [Keyring](../framework/keyring-interface.md)
        if self.source.is_none() {
            Err(val_err("A Materials Source (keyring or CMM) must be provided."))
        //= specification/client-apis/decrypt.md#input
        //# The following inputs to this behavior MUST be OPTIONAL:
        //# - [Encryption Context](#encryption-context)
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
    /// Construct a `DecryptStreamInput` with a legacy `CryptographicMaterialsManagerRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_cmm(ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptStreamInput` with a `KeyringRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_legacy_keyring(ec: EncryptionContext, keyring: LegacyKeyring) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyKeyring(keyring)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptStreamInput` with a `CryptographicMaterialsManagerRef`
    #[must_use]
    pub fn with_cmm(ec: EncryptionContext, cmm: CryptographicMaterialsManagerRef) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::Cmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptStreamInput` with a `KeyringRef`
    #[must_use]
    #[cfg(feature = "legacy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "legacy")))]
    pub fn with_keyring(ec: EncryptionContext, keyring: KeyringRef) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::Keyring(keyring)),
            ..Default::default()
        }
    }

    /// Construct a `DecryptStreamInput` from an `EncryptStreamInput`
    #[must_use]
    pub fn from_encrypt(e: &EncryptInput<'_>) -> Self {
        Self {
            encryption_context: e.encryption_context.clone(),
            source: e.source.clone(),
            commitment_policy: e.commitment_policy,
            max_encrypted_data_keys: e.max_encrypted_data_keys,
            ..Default::default()
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.source.is_none() {
            Err(val_err("A Materials Source must be provided."))
        } else {
            Ok(())
        }
    }
}
