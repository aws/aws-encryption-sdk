// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Key derivation for the ESDK message encryption key.

use super::{Error, val_err};
use crate::message::serializable_types::get_encrypt_key_length;
use aws_mpl_legacy::suites::AlgorithmSuite;
use aws_mpl_legacy::suites::DerivationAlgorithm;
use zeroize::Zeroizing;

/// Convenience container to hold both a data key and an optional commitment key
/// to support algorithm suites that provide commitment and those that do not.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpandedKeyMaterial {
    /// The derived data encryption key.
    pub data_key: Zeroizing<Vec<u8>>,
    /// The derived commitment key, present only for committing algorithm suites.
    pub commitment_key: Option<Zeroizing<Vec<u8>>>,
}

// Returns the HKDF-derived (output) key length defined by the algorithm suite.
// Only HKDF suites carry this length; any other KDF reaching here is a config error.
fn get_kdf_output_len(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.output_key_length),
        _ => Err(val_err(
            "Algorithm suite KDF must be HKDF to derive output key length",
        )),
    }
}

// Returns the HKDF input (plaintext data key) length defined by the algorithm suite.
// Only HKDF suites carry this length; any other KDF reaching here is a config error.
fn get_kdf_input_len(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.input_key_length),
        _ => Err(val_err(
            "Algorithm suite KDF must be HKDF to derive input key length",
        )),
    }
}

// Sanity-checks that a derivation algorithm is consistent with the suite and the
// supplied key length. The identity KDF uses the input key verbatim, so its length
// must already equal the suite's encryption key length. HKDF (and other) suites
// validate their lengths at derivation time, so they pass here.
const fn valid_derivation_alg(
    alg: &DerivationAlgorithm,
    suite: &AlgorithmSuite,
    key_len: usize,
) -> bool {
    match alg {
        DerivationAlgorithm::Hkdf(_x) => true,
        DerivationAlgorithm::Identity => key_len == get_encrypt_key_length(suite) as usize,
        _ => true,
    }
}

// Output length in bytes of each supported hash. Used to size the all-zero HKDF
// salt for V1 derivation (the salt length equals the hash output length, RFC 5869).
fn digest_length(alg: aws_mpl_legacy::primitives::DigestAlg) -> Result<usize, Error> {
    match alg {
        aws_mpl_legacy::primitives::DigestAlg::Sha256 => Ok(32),
        aws_mpl_legacy::primitives::DigestAlg::Sha384 => Ok(48),
        aws_mpl_legacy::primitives::DigestAlg::Sha512 => Ok(64),
        _ => Err(val_err("Unknown DigestAlg")),
    }
}

// Derives the message encryption key for V1 suites: a single HKDF call, with no
// key commitment.
pub(crate) fn derive_key_v1(
    message_id: &[u8],
    plaintext_data_key: &[u8],
    suite: &AlgorithmSuite,
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for V1 algorithms.
    debug_assert!(suite.message_version == 1);
    debug_assert!(valid_derivation_alg(
        &suite.kdf,
        suite,
        plaintext_data_key.len()
    ));

    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    match &suite.kdf {
        //= spec/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
        //# then the derived data key MUST be the same as the plaintext data key.
        //= spec/client-apis/decrypt.md#get-the-decryption-materials
        //# If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
        //# then the derived data key MUST be the same as the plaintext data key.
        DerivationAlgorithm::Identity => Ok(ExpandedKeyMaterial {
            data_key: Zeroizing::new(plaintext_data_key.to_vec()),
            commitment_key: None,
        }),
        //= spec/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
        //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
        DerivationAlgorithm::Hkdf(hkdf) => {
            let alg = hkdf.hmac;
            let salt = vec![0u8; digest_length(alg)?];
            let mut derived_key = vec![0u8; usize::try_from(hkdf.output_key_length).map_err(|_| val_err("HKDF output_key_length exceeds usize"))?];

            // The Net v4.0.0 retry path omits the suite's binary ID from the HKDF
            // info; the standard path prefixes it.
            let v4_info: [&[u8]; 1] = [message_id];
            let standard_info: [&[u8]; 2] = [&suite.binary_id[..], message_id];
            let info: &[&[u8]] = if on_net_v4_retry { &v4_info } else { &standard_info };

            aws_mpl_legacy::primitives::hkdf(alg, &salt, plaintext_data_key, info, &mut derived_key)?;

            Ok(ExpandedKeyMaterial {
                data_key: Zeroizing::new(derived_key),
                commitment_key: None,
            })
        }
        DerivationAlgorithm::None => Err(val_err("None is not a valid Key Derivation Function")),
        _ => Err(val_err("Unknown is not a valid Key Derivation Function")),
    }
}

// HKDF-expand info labels for V2 derivation: COMMITKEY produces the commitment
// key, DERIVEKEY (prefixed with the suite's binary ID) produces the encryption key.
const COMMIT_LABEL: &str = "COMMITKEY";
const KEY_LABEL: &str = "DERIVEKEY";

// Derives the message encryption key and key commitment value for V2 suites:
// one HKDF-extract followed by two HKDF-expands.
pub(crate) fn derive_key_v2(
    message_id: &[u8],
    plaintext_data_key: &[u8],
    suite: &AlgorithmSuite,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for V2 algorithms.
    if suite.message_version != 2 {
        return Err(val_err("derive_key_v2 requires message version 2"));
    }
    // For V2 algorithms, the KDF can only be HKDF.
    if u32::from(get_encrypt_key_length(suite)) != get_kdf_output_len(suite)? {
        return Err(val_err(
            "Encrypt key length must match KDF output key length",
        ));
    }
    if message_id.is_empty() {
        return Err(val_err("Message ID must not be empty"));
    }
    if plaintext_data_key.len() != usize::try_from(get_kdf_input_len(suite)?).map_err(|_| val_err("KDF input_key_length exceeds usize"))? {
        return Err(val_err(
            "Plaintext key length must match KDF input key length",
        ));
    }

    let (alg, commit_len) = match &suite.commitment {
        DerivationAlgorithm::Hkdf(hkdf) => (hkdf.hmac, hkdf.output_key_length),
        DerivationAlgorithm::None => {
            return Err(val_err("None is not a valid Commitment Algorithm"));
        }
        _ => {
            return Err(val_err("Unknown is not a valid Commitment Algorithm"));
        }
    };
    let info = [&suite.binary_id[..], KEY_LABEL.as_bytes()];

    // V2 key commitment derivation. A single HKDF-extract over the message ID (used
    // as the salt) and the plaintext key yields one pseudo-random key, which is then
    // expanded twice with distinct info labels: `binary_id || DERIVEKEY` for the
    // message encryption key (binding it to the algorithm suite), and `COMMITKEY` for
    // the commitment key. See `framework/algorithm-suites.md`.
    let pseudo_random_key =
        aws_mpl_legacy::primitives::hkdf_extract(alg, message_id, plaintext_data_key);
    let mut encrypt_key = vec![0u8; usize::try_from(get_kdf_output_len(suite)?).map_err(|_| val_err("KDF output_key_length exceeds usize"))?];
    let mut commit_key = vec![0u8; usize::try_from(commit_len).map_err(|_| val_err("Commit key length exceeds usize"))?];
    aws_mpl_legacy::primitives::hkdf_expand(&pseudo_random_key, &info, &mut encrypt_key)?;
    aws_mpl_legacy::primitives::hkdf_expand(
        &pseudo_random_key,
        &[COMMIT_LABEL.as_bytes()],
        &mut commit_key,
    )?;

    Ok(ExpandedKeyMaterial {
        data_key: Zeroizing::new(encrypt_key),
        commitment_key: Some(Zeroizing::new(commit_key)),
    })
}

/// Derives key material for encryption/decryption. Delegates to the V1 or V2
/// routine based on the algorithm suite's message version.
pub fn derive_keys(
    message_id: &[u8],
    plaintext_data_key: &[u8],
    suite: &AlgorithmSuite,
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    debug_assert!(!message_id.is_empty());
    debug_assert!(usize::from(get_encrypt_key_length(suite)) == plaintext_data_key.len());

    match suite.message_version {
        1 => derive_key_v1(message_id, plaintext_data_key, suite, on_net_v4_retry),
        2 => derive_key_v2(message_id, plaintext_data_key, suite),
        _ => Err(val_err("Unknown Message Version")),
    }
}
