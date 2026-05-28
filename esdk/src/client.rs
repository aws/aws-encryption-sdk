// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Optional [`Esdk`] client and its [`EsdkConfig`].
//!
//! `Esdk` is an optional convenience layer over the free
//! [`encrypt`](crate::encrypt) / [`decrypt`](crate::decrypt) /
//! [`encrypt_stream`](crate::encrypt_stream) /
//! [`decrypt_stream`](crate::decrypt_stream) functions. It holds an
//! [`EsdkConfig`] — a commitment policy and an optional cap on the number of
//! encrypted data keys — and forwards each call to the corresponding free
//! function with those fields filled in from the client.
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
/// Holds the commitment policy and the optional cap on the number of encrypted
/// data keys per message. Construct via [`EsdkConfig::default`] and field
/// assignment, or via [`Esdk::builder`].
//= spec/client-apis/client.md#initialization
//= reason=field defaults to None via derive(Default); None means no cap
//# If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
//# the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
//
//= spec/client-apis/client.md#initialization
//= reason=derive(Default) yields EsdkCommitmentPolicy::default() = RequireEncryptRequireDecrypt
//# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct EsdkConfig {
    /// Commitment policy applied to every operation. Default is
    /// `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`.
    //= spec/client-apis/client.md#commitment-policy
    //= reason=field type EsdkCommitmentPolicy is from aws_mpl_legacy::commitment (the MPL)
    //# The AWS Encryption SDK MUST use the ESDK [commitment policies](../framework/commitment-policy.md) defined in the Material Providers Library.
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
/// Note: `input.commitment_policy` set to its default value is not detected as
/// a conflict; configure it via [`Esdk::builder`] only.
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
    ///
    /// Private to enforce construction via [`Esdk::new`] / [`Esdk::builder`];
    /// read via [`Esdk::config`].
    //= spec/client-apis/client.md#initialization
    //= type=implication
    //= reason=pub(crate) prevents external mutation; Esdk is Copy so any "change" must construct a new value
    //# Once a [commitment policy](#commitment-policy) has been set it SHOULD be immutable.
    pub(crate) config: EsdkConfig,
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

    /// Borrow this client's configuration.
    #[must_use]
    pub const fn config(&self) -> &EsdkConfig {
        &self.config
    }

    /// Encrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some` (config provided
    /// in two places). Otherwise, returns whatever
    /// [`encrypt`](crate::encrypt) returns.
    //= spec/client-apis/client.md#encrypt
    //# The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
    //# that adheres to [encrypt](./encrypt.md).
    pub async fn encrypt(&self, input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
        crate::encrypt::encrypt(&fill_or_reject(input, &self.config)?).await
    }

    /// Decrypt with the client's configured commitment policy and EDK cap.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if `input.commitment_policy` is not the
    /// default or `input.max_encrypted_data_keys` is `Some`. Otherwise,
    /// returns whatever [`decrypt`](crate::decrypt) returns.
    //= spec/client-apis/client.md#decrypt
    //# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
    //# that adheres to [decrypt](./decrypt.md).
    pub async fn decrypt(&self, input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
        crate::decrypt::decrypt(&fill_or_reject(input, &self.config)?).await
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
        crate::encrypt::encrypt_stream(plaintext, ciphertext, &fill_or_reject(input, &self.config)?).await
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
        crate::decrypt::decrypt_stream(ciphertext, plaintext, &fill_or_reject(input, &self.config)?).await
    }
}

/// Internal abstraction over the four input structs that carry the two
/// "client-config" fields. Used by [`fill_or_reject`] so the four `Esdk`
/// methods don't repeat the rejection-and-fill logic.
trait HasClientConfigFields {
    fn client_commitment_policy(&self) -> EsdkCommitmentPolicy;
    fn client_max_encrypted_data_keys(&self) -> Option<NonZeroUsize>;
    fn set_client_commitment_policy(&mut self, p: EsdkCommitmentPolicy);
    fn set_client_max_encrypted_data_keys(&mut self, n: Option<NonZeroUsize>);
}

macro_rules! impl_has_client_config_fields {
    ($t:ty) => {
        impl HasClientConfigFields for $t {
            fn client_commitment_policy(&self) -> EsdkCommitmentPolicy {
                self.commitment_policy
            }
            fn client_max_encrypted_data_keys(&self) -> Option<NonZeroUsize> {
                self.max_encrypted_data_keys
            }
            fn set_client_commitment_policy(&mut self, p: EsdkCommitmentPolicy) {
                self.commitment_policy = p;
            }
            fn set_client_max_encrypted_data_keys(&mut self, n: Option<NonZeroUsize>) {
                self.max_encrypted_data_keys = n;
            }
        }
    };
}

impl_has_client_config_fields!(EncryptInput<'_>);
impl_has_client_config_fields!(DecryptInput<'_>);
impl_has_client_config_fields!(EncryptStreamInput);
impl_has_client_config_fields!(DecryptStreamInput);

/// Reject the call if the input also carries client-level config; otherwise
/// return a clone of the input with the client config's fields filled in.
///
/// Returning an error today (rather than picking a precedence rule) is
/// forward-compatible: a future release may define override semantics
/// without breaking callers.
fn fill_or_reject<I>(input: &I, config: &EsdkConfig) -> Result<I, Error>
where
    I: HasClientConfigFields + Clone,
{
    if input.client_commitment_policy() != EsdkCommitmentPolicy::default()
        || input.client_max_encrypted_data_keys().is_some()
    {
        return Err(val_err(
            "EsdkConfig is provided by both the Esdk client and the input struct. \
             Set commitment_policy and max_encrypted_data_keys in exactly one place: \
             on the Esdk client (via Esdk::builder), or on the input struct (when \
             using the free encrypt/decrypt functions).",
        ));
    }
    let mut filled = input.clone();
    filled.set_client_commitment_policy(config.commitment_policy);
    filled.set_client_max_encrypted_data_keys(config.max_encrypted_data_keys);
    Ok(filled)
}

/// Fluent builder for [`Esdk`].
#[derive(Debug, Clone, Copy, Default)]
pub struct EsdkBuilder {
    commitment_policy: Option<EsdkCommitmentPolicy>,
    max_encrypted_data_keys: Option<NonZeroUsize>,
}

impl EsdkBuilder {
    /// Set the commitment policy.
    //= spec/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    #[must_use]
    pub const fn commitment_policy(mut self, p: EsdkCommitmentPolicy) -> Self {
        self.commitment_policy = Some(p);
        self
    }

    /// Set the maximum number of encrypted data keys per message.
    //= spec/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
    #[must_use]
    pub const fn max_encrypted_data_keys(mut self, n: NonZeroUsize) -> Self {
        self.max_encrypted_data_keys = Some(n);
        self
    }

    /// Build the `Esdk`. Unset fields use their defaults
    /// (`RequireEncryptRequireDecrypt` commitment policy, no EDK cap).
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
