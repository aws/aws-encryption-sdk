// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use aws_mpl_rs::suites::AlgorithmSuite;
use aws_mpl_rs::suites::DerivationAlgorithm;

// Convenience container to hold both a data key and an optional commitment key
// to support algorithm suites that provide commitment and those that do not
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ExpandedKeyMaterial {
    pub(crate) data_key: Vec<u8>,
    pub(crate) commitment_key: Option<Vec<u8>>,
}

fn get_kdf_outlen(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.output_key_length),
        _ => Err("Validation Error 3".into()),
    }
}

fn get_kdf_inlen(suite: &AlgorithmSuite) -> Result<u32, Error> {
    match suite.kdf {
        DerivationAlgorithm::Hkdf(x) => Ok(x.input_key_length),
        _ => Err("Validation Error 4".into()),
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

fn digest_length(alg: aws_mpl_primitives::DigestAlg) -> Result<usize, Error> {
    match alg {
        aws_mpl_primitives::DigestAlg::Sha256 => Ok(32),
        aws_mpl_primitives::DigestAlg::Sha384 => Ok(48),
        aws_mpl_primitives::DigestAlg::Sha512 => Ok(64),
        _ => Err("Unknown DigestAlg".into()),
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

    match &suite.kdf {
        DerivationAlgorithm::Identity => Ok(ExpandedKeyMaterial {
            data_key: plaintext_data_key.to_vec(),
            commitment_key: None,
        }),
        DerivationAlgorithm::Hkdf(hkdf) => {
            let alg = hkdf.hmac;
            let salt = vec![0u8; digest_length(alg)?];
            let mut derived_key = vec![0u8; hkdf.output_key_length as usize];
            if on_net_v4_retry {
                aws_mpl_primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    &[message_id],
                    &mut derived_key,
                )?;
            } else {
                aws_mpl_primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    &[&suite.binary_id[..], message_id],
                    &mut derived_key,
                )?;
            }

            Ok(ExpandedKeyMaterial {
                data_key: derived_key,
                commitment_key: None,
            })
        }
        DerivationAlgorithm::None => Err("None is not a valid Key Derivation Function".into()),
        _ => Err("Unknown is not a valid Key Derivation Function".into()),
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
        return Err("Validation Error 8".into());
    }
    // For v2 algorithms, KDF can only be HKDF
    if u32::from(get_encrypt_key_length(suite)) != get_kdf_outlen(suite)? {
        return Err("Validation Error 9".into());
    }
    if message_id.is_empty() {
        return Err("Validation Error 10".into());
    }
    if plaintext_key.len() as u32 != get_kdf_inlen(suite)? {
        return Err("Validation Error 11".into());
    }

    let (digest, commit_len) = match &suite.commitment {
        DerivationAlgorithm::Hkdf(hkdf) => (hkdf.hmac, hkdf.output_key_length),
        DerivationAlgorithm::None => {
            return Err("None is not a valid Commitment Algorithm".into());
        }
        _ => {
            return Err("Unknown is not a valid Commitment Algorithm".into());
        }
    };
    let alg = digest;
    let info = [&suite.binary_id[..], KEY_LABEL.as_bytes()];

    let pseudo_random_key = aws_mpl_primitives::hkdf_extract(alg, message_id, plaintext_key);
    let mut encrypt_key = vec![0u8; get_kdf_outlen(suite)? as usize];
    let mut commit_key = vec![0u8; commit_len as usize];
    aws_mpl_primitives::hkdf_expand(&pseudo_random_key, &info, &mut encrypt_key)?;
    aws_mpl_primitives::hkdf_expand(
        &pseudo_random_key,
        &[COMMIT_LABEL.as_bytes()],
        &mut commit_key,
    )?;

    Ok(ExpandedKeyMaterial {
        data_key: encrypt_key,
        commitment_key: Some(commit_key),
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
        Err("Unknown Message Version".into())
    }
}
