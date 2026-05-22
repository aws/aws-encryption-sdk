// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Public API types for encrypt and decrypt operations.

use crate::Error;
use crate::val_err;
use aws_mpl_legacy::CryptographicMaterialsManagerRef;
use aws_mpl_legacy::KeyringRef;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef as LegacyCMM;
use aws_mpl_legacy::dafny::types::keyring::KeyringRef as LegacyKeyring;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use std::num::NonZeroUsize;

/// Source for Cryptographic Materials
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MaterialSource {
    /// Legacy CMM, i.e. `aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef`
    LegacyCmm(LegacyCMM),
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
            (Self::LegacyCmm(x), Self::LegacyCmm(y)) => x == y,
            (Self::LegacyKeyring(x), Self::LegacyKeyring(y)) => x == y,
            _ => false,
        }
    }
}
impl Eq for MaterialSource {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The length of one frame, must be non-zero, defaults to 4096.
//= spec/data-format/message-body.md#framed-data
//= reason=Max value of a U32 is 2^32 - 1; enforced by construction
//# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
#[expect(clippy::exhaustive_structs)]
pub struct FrameLength(pub std::num::NonZeroU32);

impl Default for FrameLength {
    fn default() -> Self {
        //= spec/client-apis/encrypt.md#frame-length
        //# This value MUST default to 4096 bytes.
        // Safety: 4096 is a non-zero literal
        Self(std::num::NonZeroU32::new(4096).expect("4096 is non-zero"))
    }
}

impl FrameLength {
    /// return new `FrameLength`.
    ///
    /// # Errors
    /// Returns an error if `val` is zero.
    pub fn new(val: u32) -> Result<Self, Error> {
        //= spec/client-apis/encrypt.md#frame-length
        //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
        Ok(Self(
            std::num::NonZeroU32::new(val)
                .ok_or_else(|| val_err("Frame length must be non-zero"))?,
        ))
    }
    /// return new `FrameLength`. Panic if val is zero.
    ///
    /// # Panics
    /// Panics if `val` is zero.
    #[must_use]
    pub const fn new_unchecked(val: u32) -> Self {
        Self(std::num::NonZeroU32::new(val).unwrap())
    }
}

/// Convenience function to return a `MaterialProviders` Client.
///
/// # Panics
/// Panics if the `MaterialProviders` client cannot be constructed.
#[must_use]
pub fn mpl() -> aws_mpl_legacy::dafny::client::Client {
    aws_mpl_legacy::dafny::client::Client::from_conf(
        aws_mpl_legacy::dafny::types::MaterialProvidersConfig::builder()
            .build()
            .unwrap(),
    )
    .unwrap()
}

/// Output bytes sink for streaming encrypt/decrypt operations.
///
/// Auto-implemented for any type that is `Write + Send + Sync + Debug`
/// (e.g. `Vec<u8>`, `std::fs::File`).
//= spec/client-apis/streaming.md#outputs
//# In order to support streaming, the operation MUST produce some output within a streaming framework.
//
//= spec/client-apis/streaming.md#outputs
//= reason=SafeWrite wraps std::io::Write; write() pushes bytes to the consumer immediately
//# - There MUST be a mechanism for output bytes to be released.
pub trait SafeWrite: std::io::Write + Send + Sync + std::fmt::Debug {}
//= spec/client-apis/streaming.md#outputs
//= reason=SafeWrite wraps std::io::Write; the operation returning Ok(()) signals that all output has been written
//# - There MUST be a mechanism to indicate that the entire output has been released.
impl<T: std::io::Write + Send + Sync + std::fmt::Debug> SafeWrite for T {}

/// Input bytes source for streaming encrypt/decrypt operations.
///
/// Auto-implemented for any type that is `Read + Send + Sync + Debug`
/// (e.g. `std::io::Cursor`, `std::fs::File`).
//= spec/client-apis/streaming.md#inputs
//# In order to support streaming, the operation MUST accept some input within a streaming framework.
//
//= spec/client-apis/streaming.md#inputs
//= reason=SafeRead wraps std::io::Read; read() returns bytes as they become available
//# - There MUST be a mechanism for input bytes to become consumable.
pub trait SafeRead: std::io::Read + Send + Sync + std::fmt::Debug {}
//= spec/client-apis/streaming.md#overview
//= reason=SafeRead wraps std::io::Read, enabling incremental consumption; the implementation does not require holding the entire input in memory
//# If an implementation requires holding the entire input in memory in order to perform the operation,
//# that implementation SHOULD NOT provide an API that allows the caller to stream the operation.
//
//= spec/client-apis/streaming.md#inputs
//= reason=SafeRead wraps std::io::Read; read() returning Ok(0) signals EOF
//# - There MUST be a mechanism to indicate that there are no more input bytes.
impl<T: std::io::Read + Send + Sync + std::fmt::Debug> SafeRead for T {}

/// Key-Value pairs to associate with the encrypted data
pub use aws_mpl_legacy::EncryptionContext;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
/// Output for [`encrypt`](crate::encrypt).
//= spec/client-apis/encrypt.md#output
//= type=exception
//= reason=EncryptOutput provides encryption_context and algorithm_suite_id individually instead of a parsed header
//# - Encrypt operation output SHOULD include a [Parsed Header](#parsed-header) value.
//
//= spec/client-apis/encrypt.md#algorithm-suite-1
//= type=exception
//= reason=EncryptOutput returns algorithm_suite_id directly rather than via a parsed header
//# This output MAY be satisfied by outputting a [parsed header](#parsed-header) containing this value.
//
//= spec/client-apis/encrypt.md#encryption-context-1
//= type=exception
//= reason=EncryptOutput returns encryption_context directly rather than via a parsed header
//# This output MAY be satisfied by outputting a [parsed header](#parsed-header) containing this value.
pub struct EncryptOutput {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    pub algorithm_suite_id: EsdkAlgorithmSuiteId,
    /// Encrypted message bytes (the serialized AWS Encryption SDK message).
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
    /// Decrypted plaintext data
    //= spec/client-apis/decrypt.md#plaintext
    //# This MUST be a sequence of bytes.
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
//= spec/client-apis/encrypt.md#input
//= reason=EncryptInput has plaintext: &[u8] (always known length) and no plaintext_length_bound field, so a caller cannot specify both
//# Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext)
//# with known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction.
//
//= spec/client-apis/encrypt.md#input
//= reason=EncryptInput has plaintext: &[u8] (always known length) and no plaintext_length_bound field, making it impossible to specify both
//# If a caller is able to specify both an input [plaintext](#plaintext) with known length and
//# a [Plaintext Length Bound](#plaintext-length-bound),
//# the [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation
//# and MUST be ignored.
pub struct EncryptInput<'a> {
    /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
    //= spec/client-apis/encrypt.md#input
    //# - Encrypt operation input MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    /// Key-Value pairs to associate with the encrypted data
    //= spec/client-apis/encrypt.md#input
    //# - Encrypt operation input MUST accept an optional [Encryption Context](#encryption-context) argument.
    pub encryption_context: EncryptionContext,
    /// Bytes of plaintext data per frame. Default 4096.
    //= spec/client-apis/encrypt.md#input
    //# - Encrypt operation input MUST accept an optional [Frame Length](#frame-length) argument.
    pub frame_length: FrameLength,
    /// The source of cryptographic materials
    //= spec/client-apis/encrypt.md#input
    //# - Encrypt operation input MUST accept an optional [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) argument.
    //
    //= spec/client-apis/encrypt.md#input
    //= reason=source is Option<MaterialSource>, making CMM/keyring optional by construction
    //# - Encrypt operation input MUST accept an optional [keyring](../framework/keyring-interface.md) argument.
    pub source: Option<MaterialSource>,
    /// data to be encrypted
    //= spec/client-apis/encrypt.md#input
    //# - Encrypt operation input MUST accept a required [plaintext](#plaintext) argument.
    //
    //= spec/client-apis/encrypt.md#plaintext
    //# This MUST be a sequence of bytes.
    pub plaintext: &'a [u8],
    /// Default is no limit
    //= spec/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
    //
    //= spec/client-apis/client.md#initialization
    //# If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
    //# the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// Default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
    //= spec/client-apis/client.md#commitment-policy
    //= reason=commitment_policy field type is EsdkCommitmentPolicy from the Material Providers Library
    //# The AWS Encryption SDK MUST use the ESDK [commitment policies](../framework/commitment-policy.md) defined in the Material Providers Library.
    //
    //= spec/client-apis/client.md#initialization
    //= reason=EsdkCommitmentPolicy's Default impl returns RequireEncryptRequireDecrypt; EncryptInput uses #[derive(Default)]
    //# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
    //
    //= spec/client-apis/client.md#initialization
    //= reason=once the struct is consumed by encrypt()/decrypt(), the caller cannot mutate the policy
    //# Once a [commitment policy](#commitment-policy) has been set it SHOULD be immutable.
    //
    //= spec/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    pub commitment_policy: EsdkCommitmentPolicy,
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
        //= spec/client-apis/encrypt.md#input
        //# The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
        if self.source.is_none() {
            //= spec/client-apis/encrypt.md#input
            //# If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.
            Err(val_err("A materials source must be provided"))
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
    /// Optional expected size of the input plaintext stream.
    ///
    /// Most CMMs and keyrings ignore this; provide it only when the configured
    /// CMM or keyring requires the plaintext length up front (e.g. for a
    /// signing scheme that pre-computes a digest size).
    //= spec/client-apis/encrypt.md#input
    //= reason=EncryptStreamInput accepts unknown-length plaintext via a stream; data_size serves as the optional Plaintext Length Bound
    //# If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a
    //# [Plaintext Length Bound](#plaintext-length-bound).
    pub data_size: Option<usize>,
    /// Default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// Default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
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
    pub fn with_legacy_cmm(ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct an `EncryptStreamInput` with a legacy `KeyringRef`
    #[must_use]
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
        if self.source.is_none() {
            Err(val_err("A materials source must be provided"))
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
    //= spec/client-apis/decrypt.md#input
    //# - Decrypt operation input MUST accept a required [Encrypted Message](#encrypted-message) argument.
    //
    //= spec/client-apis/decrypt.md#encrypted-message
    //# The input encrypted message MUST be a sequence of bytes in the
    //# [message format](../data-format/message.md) specified by the AWS Encryption SDK.
    pub ciphertext: &'a [u8],
    /// Key-Value pairs to associate with the encrypted data
    //= spec/client-apis/decrypt.md#input
    //# - Decrypt operation input MUST accept an optional [Encryption Context](#encryption-context) argument.
    pub encryption_context: EncryptionContext,
    /// The source of cryptographic materials
    //= spec/client-apis/decrypt.md#input
    //# - Decrypt operation input MUST accept an optional [Cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) argument.
    //
    //= spec/client-apis/decrypt.md#input
    //# - Decrypt operation input MUST accept an optional [Keyring](../framework/keyring-interface.md) argument.
    pub source: Option<MaterialSource>,
    /// Default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// Default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// Default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
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
    /// If you decrypt a signed payload, most of the data will be written
    /// to the output stream before the signature is verified.
    /// Thus, if verification fails, you are responsible for discarding any data
    /// already received. If you are willing to accept this, set `i_accept_the_danger` to true.
    /// If verification fails, at least one byte will not have been written to the output stream.
    /// If the ciphertext involves only one frame, then no danger exists, and this flag is not needed.
    pub i_accept_the_danger: bool,
    /// Default is `NetV400RetryPolicy::AllowRetry`
    pub net_v4_retry_policy: NetV400RetryPolicy,
    /// Default is no limit
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
    /// Default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
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
        //= spec/client-apis/decrypt.md#input
        //# The Decrypt operation MUST validate that exactly one of a keyring or CMM was provided by the caller.
        if self.source.is_none() {
            //= spec/client-apis/decrypt.md#input
            //# If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
            Err(val_err("A materials source must be provided"))
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
    pub fn with_legacy_cmm(ec: EncryptionContext, cmm: LegacyCMM) -> Self {
        Self {
            encryption_context: ec,
            source: Some(MaterialSource::LegacyCmm(cmm)),
            ..Default::default()
        }
    }
    /// Construct a `DecryptStreamInput` with a `KeyringRef`
    #[must_use]
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
            Err(val_err("A materials source must be provided"))
        } else {
            Ok(())
        }
    }
}
