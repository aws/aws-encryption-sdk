// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use aws_mpl_legacy::types::AlgorithmSuiteInfo;
use aws_mpl_legacy::types::DerivationAlgorithm;

// Convenience container to hold both a data key and an optional commitment key
// to support algorithm suites that provide commitment and those that do not
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(crate) struct ExpandedKeyMaterial {
    pub(crate) data_key: Vec<u8>,
    pub(crate) commitment_key: Option<Vec<u8>>,
}

fn get_kdf_outlen(suite: &AlgorithmSuiteInfo) -> Result<i32, Error> {
    match suite.kdf.as_ref().unwrap() {
        DerivationAlgorithm::Hkdf(x) => Ok(x.output_key_length.unwrap()),
        _ => Err("Validation Error 3".into()),
    }
}

fn get_kdf_inlen(suite: &AlgorithmSuiteInfo) -> Result<i32, Error> {
    match suite.kdf.as_ref().unwrap() {
        DerivationAlgorithm::Hkdf(x) => Ok(x.input_key_length.unwrap()),
        _ => Err("Validation Error 4".into()),
    }
}

const fn valid_derivation_alg(
    alg: &DerivationAlgorithm,
    suite: &AlgorithmSuiteInfo,
    key_len: usize,
) -> bool {
    match alg {
        DerivationAlgorithm::Hkdf(_x) => true,
        DerivationAlgorithm::Identity(_x) => key_len == get_encrypt_key_length(suite) as usize,
        _ => true,
    }
}

const fn get_hkdf_alg(
    alg: aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm,
) -> aws_mpl_primitives::DigestAlg {
    match alg {
        aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm::Sha256 => {
            aws_mpl_primitives::DigestAlg::Sha256
        }
        aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm::Sha384 => {
            aws_mpl_primitives::DigestAlg::Sha384
        }
        aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm::Sha512 => {
            aws_mpl_primitives::DigestAlg::Sha512
        }
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
    suite: &AlgorithmSuiteInfo,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for v1 algorithms
    debug_assert!(suite.message_version.unwrap() == 1);
    debug_assert!(valid_derivation_alg(
        suite.kdf.as_ref().unwrap(),
        suite,
        plaintext_data_key.len()
    ));

    //= compliance/client-apis/encrypt.txt#2.6.1
    //# The algorithm used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) defined
    //# above.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# The algorithm suite used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) associated
    //# with the returned decryption materials.
    match &suite.kdf.as_ref().unwrap() {
        DerivationAlgorithm::Identity(_i) => Ok(ExpandedKeyMaterial {
            data_key: plaintext_data_key.to_vec(),
            commitment_key: None,
        }),
        DerivationAlgorithm::Hkdf(hkdf) => {
            let alg = get_hkdf_alg(hkdf.hmac.unwrap());
            let salt = vec![0u8; digest_length(alg)?];
            let mut derived_key = vec![0u8; hkdf.output_key_length.unwrap() as usize];
            if on_net_v4_retry {
                aws_mpl_primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    message_id,
                    &mut derived_key,
                )?;
            } else {
                aws_mpl_primitives::hkdf(
                    alg,
                    &salt,
                    plaintext_data_key,
                    &[suite.binary_id.as_ref().unwrap().as_ref(), message_id].concat(),
                    &mut derived_key,
                )?;
            }

            Ok(ExpandedKeyMaterial {
                data_key: derived_key,
                commitment_key: None,
            })
        }
        DerivationAlgorithm::None(_x) => Err("None is not a valid Key Derivation Function".into()),
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
    suite: &AlgorithmSuiteInfo,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for v2 algorithms
    if suite.message_version.unwrap() != 2 {
        return Err("Validation Error 8".into());
    }
    // For v2 algorithms, KDF can only be HKDF
    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# The algorithm suite used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) associated
    //# with the returned decryption materials.
    if i32::from(get_encrypt_key_length(suite)) != get_kdf_outlen(suite)? {
        return Err("Validation Error 9".into());
    }
    if message_id.is_empty() {
        return Err("Validation Error 10".into());
    }
    //= compliance/client-apis/encrypt.txt#2.6.1
    //# The algorithm used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) defined
    //# above.
    #[allow(clippy::cast_possible_wrap)]
    if plaintext_key.len() as i32 != get_kdf_inlen(suite)? {
        return Err("Validation Error 11".into());
    }

    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# If the algorithm suite (../framework/
    //# algorithm-suites.md#algorithm-suites-encryption-key-derivation-
    //# settings) supports key commitment (../framework/algorithm-
    //# suites.md#key-commitment) then the commit key (../framework/
    //# algorithm-suites.md#commit-key) MUST be derived from the plaintext
    //# data key using the commit key derivation (../framework/algorithm-
    //# suites.md#algorithm-suites-commit-key-derivation-settings).

    let (digest, commit_len) = match &suite.commitment.as_ref().unwrap() {
        DerivationAlgorithm::Hkdf(hkdf) => (hkdf.hmac.unwrap(), hkdf.output_key_length.unwrap()),
        DerivationAlgorithm::None(_x) => {
            return Err("None is not a valid Commitment Algorithm".into());
        }
        _ => {
            return Err("Unknown is not a valid Commitment Algorithm".into());
        }
    };
    let alg = get_hkdf_alg(digest);
    let info = [
        suite.binary_id.as_ref().unwrap().as_ref(),
        KEY_LABEL.as_bytes(),
    ]
    .concat();

    let pseudo_random_key = aws_mpl_primitives::hkdf_extract(alg, message_id, plaintext_key);
    let mut encrypt_key = vec![0u8; get_kdf_outlen(suite)? as usize];
    let mut commit_key = vec![0u8; commit_len as usize];
    aws_mpl_primitives::hkdf_expand(&pseudo_random_key, &info, &mut encrypt_key)?;
    aws_mpl_primitives::hkdf_expand(&pseudo_random_key, COMMIT_LABEL.as_bytes(), &mut commit_key)?;

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
    suite: &AlgorithmSuiteInfo,
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    debug_assert!(!message_id.is_empty());
    debug_assert!(get_encrypt_key_length(suite) as usize == plaintext_key.len());

    if suite.message_version.unwrap() == 2 {
        expand_key_material(message_id, plaintext_key, suite)
    } else if suite.message_version.unwrap() == 1 {
        derive_key(message_id, plaintext_key, suite, on_net_v4_retry)
    } else {
        Err("Unknown Message Version".into())
    }
}
