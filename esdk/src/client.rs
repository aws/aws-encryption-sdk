// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Optional [`Esdk`] client and its [`EsdkConfig`].
//!
//! `Esdk` is an optional convenience layer over the free
//! [`encrypt`](crate::encrypt) / [`decrypt`](crate::decrypt) /
//! [`encrypt_stream`](crate::encrypt_stream) /
//! [`decrypt_stream`](crate::decrypt_stream) functions. It holds an
//! [`EsdkConfig`] (the fields the spec defines as "client config" — commitment
//! policy and max encrypted data keys) and forwards each call to the
//! corresponding free function with those fields filled in.
//!
//! Callers who don't want a client can keep using the free functions and set
//! `commitment_policy` / `max_encrypted_data_keys` directly on the input.
//!
//! Per-call config on the input AND a configured `Esdk` is currently rejected:
//! returning an error today is forward-compatible (override semantics can be
//! defined later without breaking callers); committing to a precedence rule
//! today is not.

use crate::types::{
    DecryptInput, DecryptOutput, DecryptStreamInput, DecryptStreamOutput, EncryptInput,
    EncryptOutput, EncryptStreamInput, EncryptStreamOutput, SafeRead, SafeWrite,
};
use crate::{Error, val_err};
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use std::num::NonZeroUsize;

/// Cross-call configuration for the optional [`Esdk`] client.
///
/// Holds the fields the spec defines as belonging on the client config rather
/// than on the per-call input: [commitment policy](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/client-apis/client.md#commitment-policy)
/// and [maximum number of encrypted data keys](https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/client-apis/client.md#maximum-number-of-encrypted-data-keys).
///
/// Construct via [`EsdkConfig::default`] and field assignment, or via
/// [`Esdk::builder`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct EsdkConfig {
    /// Commitment policy applied to every operation.
    /// Default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`.
    pub commitment_policy: EsdkCommitmentPolicy,
    /// Optional cap on the number of encrypted data keys per message.
    /// Default is `None` (no cap beyond the message-format limit).
    pub max_encrypted_data_keys: Option<NonZeroUsize>,
}

/// Optional client over the free encrypt/decrypt functions.
///
/// Holds an [`EsdkConfig`] and forwards each operation to the corresponding
/// free function ([`encrypt`](crate::encrypt), [`decrypt`](crate::decrypt),
/// [`encrypt_stream`](crate::encrypt_stream), [`decrypt_stream`](crate::decrypt_stream))
/// with `commitment_policy` and `max_encrypted_data_keys` filled in from
/// `self.config`.
///
/// If the input also sets either of those fields, the call is rejected with a
/// validation error. Override semantics may be defined in a future release;
/// callers should set the fields in exactly one place.
///
/// # Example
///
/// ```ignore
/// let esdk = Esdk::builder()
///     .commitment_policy(EsdkCommitmentPolicy::RequireEncryptRequireDecrypt)
///     .build();
/// let input = EncryptInput::with_keyring(plaintext, ec, keyring);
/// let output = esdk.encrypt(&input).await?;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct Esdk {
    /// Client configuration applied to every call.
    pub config: EsdkConfig,
}

impl Esdk {
    /// Construct an `Esdk` with the given config.
    #[must_use]
    pub const fn new(config: EsdkConfig) -> Self {
        Self { config }
    }

    /// Start a fluent builder for `Esdk`.
    #[must_use]
    pub fn builder() -> EsdkBuilder {
        EsdkBuilder::default()
    }

    /// Encrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some` (config provided
    /// in two places). Otherwise, returns whatever
    /// [`encrypt`](crate::encrypt) returns.
    pub async fn encrypt(&self, input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
        reject_input_client_config(input.commitment_policy, input.max_encrypted_data_keys)?;
        let mut filled = input.clone();
        filled.commitment_policy = self.config.commitment_policy;
        filled.max_encrypted_data_keys = self.config.max_encrypted_data_keys;
        crate::encrypt::encrypt(&filled).await
    }

    /// Decrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some`. Otherwise,
    /// returns whatever [`decrypt`](crate::decrypt) returns.
    pub async fn decrypt(&self, input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
        reject_input_client_config(input.commitment_policy, input.max_encrypted_data_keys)?;
        let mut filled = input.clone();
        filled.commitment_policy = self.config.commitment_policy;
        filled.max_encrypted_data_keys = self.config.max_encrypted_data_keys;
        crate::decrypt::decrypt(&filled).await
    }

    /// Stream-encrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some`. Otherwise,
    /// returns whatever [`encrypt_stream`](crate::encrypt_stream) returns.
    pub async fn encrypt_stream(
        &self,
        plaintext: &mut dyn SafeRead,
        ciphertext: &mut dyn SafeWrite,
        input: &EncryptStreamInput,
    ) -> Result<EncryptStreamOutput, Error> {
        reject_input_client_config(input.commitment_policy, input.max_encrypted_data_keys)?;
        let mut filled = input.clone();
        filled.commitment_policy = self.config.commitment_policy;
        filled.max_encrypted_data_keys = self.config.max_encrypted_data_keys;
        crate::encrypt::encrypt_stream(plaintext, ciphertext, &filled).await
    }

    /// Stream-decrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some`. Otherwise,
    /// returns whatever [`decrypt_stream`](crate::decrypt_stream) returns.
    pub async fn decrypt_stream(
        &self,
        ciphertext: &mut dyn SafeRead,
        plaintext: &mut dyn SafeWrite,
        input: &DecryptStreamInput,
    ) -> Result<DecryptStreamOutput, Error> {
        reject_input_client_config(input.commitment_policy, input.max_encrypted_data_keys)?;
        let mut filled = input.clone();
        filled.commitment_policy = self.config.commitment_policy;
        filled.max_encrypted_data_keys = self.config.max_encrypted_data_keys;
        crate::decrypt::decrypt_stream(ciphertext, plaintext, &filled).await
    }
}

/// Reject the call if the input also carries client-level config.
///
/// We intentionally do not define override semantics today: returning an error
/// is forward-compatible (a future release may accept the call with explicit
/// precedence rules), while shipping any specific precedence today would lock
/// it in.
fn reject_input_client_config(
    commitment_policy: EsdkCommitmentPolicy,
    max_encrypted_data_keys: Option<NonZeroUsize>,
) -> Result<(), Error> {
    if commitment_policy != EsdkCommitmentPolicy::default()
        || max_encrypted_data_keys.is_some()
    {
        return Err(val_err(
            "EsdkConfig is provided by both the Esdk client and the input struct. \
             Set commitment_policy and max_encrypted_data_keys in exactly one place: \
             on the Esdk client (via Esdk::builder), or on the input struct (when \
             using the free encrypt/decrypt functions). Mixing them is reserved for \
             a future release.",
        ));
    }
    Ok(())
}

/// Fluent builder for [`Esdk`].
#[derive(Debug, Clone, Copy, Default)]
pub struct EsdkBuilder {
    commitment_policy: Option<EsdkCommitmentPolicy>,
    max_encrypted_data_keys: Option<NonZeroUsize>,
}

impl EsdkBuilder {
    /// Set the commitment policy.
    #[must_use]
    pub const fn commitment_policy(mut self, p: EsdkCommitmentPolicy) -> Self {
        self.commitment_policy = Some(p);
        self
    }

    /// Set the maximum number of encrypted data keys per message.
    #[must_use]
    pub const fn max_encrypted_data_keys(mut self, n: NonZeroUsize) -> Self {
        self.max_encrypted_data_keys = Some(n);
        self
    }

    /// Build the `Esdk`.
    #[must_use]
    pub fn build(self) -> Esdk {
        Esdk {
            config: EsdkConfig {
                commitment_policy: self.commitment_policy.unwrap_or_default(),
                max_encrypted_data_keys: self.max_encrypted_data_keys,
            },
        }
    }
}
