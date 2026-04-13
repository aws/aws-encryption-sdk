// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::{Error, val_err};
use crate::message::header_types::MessageId;
use crate::message::serializable_types::get_encrypt_key_length;
use aws_mpl_legacy::suites::AlgorithmSuite;
use aws_mpl_legacy::suites::DerivationAlgorithm;
use zeroize::Zeroizing;

// Convenience container to hold both a data key and an optional commitment key
// to support algorithm suites that provide commitment and those that do not
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedKeyMaterial {
    pub(crate) data_key: Zeroizing<Vec<u8>>,
    pub(crate) commitment_key: Option<Zeroizing<Vec<u8>>>,
}

fn get_kdf_outlen(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.output_key_length),
        _ => Err(val_err("Algorithm suite KDF must be HKDF to derive output key length")),
    }
}

fn get_kdf_inlen(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.input_key_length),
        _ => Err(val_err("Algorithm suite KDF must be HKDF to derive input key length")),
    }
}

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

fn digest_length(alg: aws_mpl_legacy::primitives::DigestAlg) -> Result<usize, Error> {
    match alg {
        aws_mpl_legacy::primitives::DigestAlg::Sha256 => Ok(32),
        aws_mpl_legacy::primitives::DigestAlg::Sha384 => Ok(48),
        aws_mpl_legacy::primitives::DigestAlg::Sha512 => Ok(64),
        _ => Err(val_err("Unknown DigestAlg")),
    }
}

// Derives a single data key from an input plaintext data key, using "v1"-style
// key derivation (that is, no key commitment).
pub(crate) fn derive_key(
    message_id: &MessageId,
    plaintext_data_key: &[u8],
    suite: &AlgorithmSuite,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for v1 algorithms
    debug_assert!(suite.message_version == 1);
    debug_assert!(valid_derivation_alg(
        &suite.kdf,
        suite,
        plaintext_data_key.len()
    ));

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    match &suite.kdf {
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
        //# then the derived data key MUST be the same as the plaintext data key.
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
        //# then the derived data key MUST be the same as the plaintext data key.
        DerivationAlgorithm::Identity => Ok(ExpandedKeyMaterial {
            data_key: Zeroizing::new(plaintext_data_key.to_vec()),
            commitment_key: None,
        }),
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
        //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
        DerivationAlgorithm::Hkdf(hkdf) => {
            let alg = hkdf.hmac;
            let salt = vec![0u8; digest_length(alg)?];
            let mut derived_key = vec![0u8; hkdf.output_key_length as usize];
            if on_net_v4_retry {
                aws_mpl_legacy::primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    &[message_id],
                    &mut derived_key,
                )?;
            } else {
                aws_mpl_legacy::primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    &[&suite.binary_id[..], message_id],
                    &mut derived_key,
                )?;
            }

            Ok(ExpandedKeyMaterial {
                data_key: Zeroizing::new(derived_key),
                commitment_key: None,
            })
        }
        DerivationAlgorithm::None => Err(val_err("None is not a valid Key Derivation Function")),
        _ => Err(val_err("Unknown is not a valid Key Derivation Function")),
    }
}

const COMMIT_LABEL: &str = "COMMITKEY";
const KEY_LABEL: &str = "DERIVEKEY";

/*
 * Derives keys from an input plaintext data key, using "v2"-style
 * key derivation (that is, including key commitment).
 */
pub(crate) fn expand_key_material(
    message_id: &MessageId,
    plaintext_key: &[u8],
    suite: &AlgorithmSuite,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for v2 algorithms
    if suite.message_version != 2 {
        return Err(val_err("expand_key_material requires message version 2"));
    }
    // For v2 algorithms, KDF can only be HKDF
    if u32::from(get_encrypt_key_length(suite)) != get_kdf_outlen(suite)? {
        return Err(val_err("Encrypt key length must match KDF output key length"));
    }
    if message_id.is_empty() {
        return Err(val_err("Message ID must not be empty"));
    }
    if plaintext_key.len() != get_kdf_inlen(suite)? as usize {
        return Err(val_err("Plaintext key length must match KDF input key length"));
    }

    let (digest, commit_len) = match &suite.commitment {
        DerivationAlgorithm::Hkdf(hkdf) => (hkdf.hmac, hkdf.output_key_length),
        DerivationAlgorithm::None => {
            return Err(val_err("None is not a valid Commitment Algorithm"));
        }
        _ => {
            return Err(val_err("Unknown is not a valid Commitment Algorithm"));
        }
    };
    let alg = digest;
    let info = [&suite.binary_id[..], KEY_LABEL.as_bytes()];

    let pseudo_random_key = aws_mpl_legacy::primitives::hkdf_extract(alg, message_id, plaintext_key);
    let mut encrypt_key = vec![0u8; get_kdf_outlen(suite)? as usize];
    let mut commit_key = vec![0u8; commit_len as usize];
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

/*
 * Derives key material for encryption/decryption. Delegates out to specific methods
 * based on the input algorithm suite.
 */
pub(crate) fn derive_keys(
    message_id: &MessageId,
    plaintext_key: &[u8],
    suite: &AlgorithmSuite,
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    debug_assert!(!message_id.is_empty());
    debug_assert!(get_encrypt_key_length(suite) as usize == plaintext_key.len());

    if suite.message_version == 2 {
        expand_key_material(message_id, plaintext_key, suite)
    } else if suite.message_version == 1 {
        derive_key(message_id, plaintext_key, suite, on_net_v4_retry)
    } else {
        Err(val_err("Unknown Message Version"))
    }
}
