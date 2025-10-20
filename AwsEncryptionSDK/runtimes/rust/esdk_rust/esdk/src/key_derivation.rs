// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use aws_mpl_rs::types::AlgorithmSuiteInfo;
use aws_mpl_rs::types::DerivationAlgorithm;

// Convenience container to hold both a data key and an optional commitment key
// to support algorithm suites that provide commitment and those that do not
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(crate) struct ExpandedKeyMaterial {
    pub(crate) data_key: Vec<u8>,
    pub(crate) commitment_key: Option<Vec<u8>>,
}
/*
    requires suite.kdf.HKDF?
             ==>
               && |plaintextDataKey| == suite.kdf.HKDF.inputKeyLength as nat
               && suite.kdf.HKDF.outputKeyLength == SerializableTypes.GetEncryptKeyLength(suite)

*/

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
// Derives a single data key from an input plaintext data key, using "v1"-style
// key derivation (that is, no key commitment).
pub(crate) async fn derive_key(
    message_id: &MessageId,
    plaintext_data_key: &[u8],
    suite: &AlgorithmSuiteInfo,
    crypto: &aws_mpl_rs::aws_cryptography_primitives::client::Client,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    // This should only be used for v1 algorithms
    if suite.message_version.unwrap() != 1 {
        return Err("Validation Error 5".into());
    }
    // if suite.commitment.is_some() {
    //     return Err("Validation Error 6".into());
    // }
    if !valid_derivation_alg(suite.kdf.as_ref().unwrap(), suite, plaintext_data_key.len()) {
        return Err("Validation Error 7".into());
    }

    // ensures res.Success? ==> |res.value.dataKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat
    // ensures res.Success? ==> IsDerivedKey(res.value.dataKey)
    // ensures res.Success? ==> res.value.commitmentKey.None?
    // ensures res.Success? ==> suite.kdf.IDENTITY? || suite.kdf.HKDF?
    // ensures suite.kdf.None? ==> res.Failure?

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
            // let hkdf_input_builder: aws_mpl_rs::aws_cryptography_primitives::types::builders::HkdfInputBuilder = aws_mpl_rs::deps::aws_cryptography_primitives::types::HkdfInput::builder()
            let hkdf_builder = crypto
                .hkdf()
                .digest_algorithm(hkdf.hmac.unwrap())
                .ikm(plaintext_data_key)
                .expected_length(hkdf.output_key_length.unwrap());
            let derived_key = if on_net_v4_retry {
                hkdf_builder
                    .info(suite.binary_id.as_ref().unwrap().as_ref())
                    .send()
                    .await
                    .unwrap()
            } else {
                hkdf_builder
                    .info([suite.binary_id.as_ref().unwrap().as_ref(), message_id].concat())
                    .send()
                    .await
                    .unwrap()
            };

            Ok(ExpandedKeyMaterial {
                data_key: derived_key.into(),
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
pub(crate) async fn expand_key_material(
    message_id: &MessageId,
    plaintext_key: &[u8],
    suite: &AlgorithmSuiteInfo,
    crypto: &aws_mpl_rs::aws_cryptography_primitives::client::Client,
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
    // ensures res.Success? ==>
    //           && res.value.commitmentKey.Some?
    //           && |res.value.commitmentKey.value| == suite.commitment.HKDF.outputKeyLength as nat

    // ensures res.Success? ==> |res.value.dataKey|  == SerializableTypes.GetEncryptKeyLength(suite) as nat

    let (digest, commit_len) = match &suite.commitment.as_ref().unwrap() {
        DerivationAlgorithm::Hkdf(hkdf) => (hkdf.hmac.unwrap(), hkdf.output_key_length.unwrap()),
        DerivationAlgorithm::None(_x) => {
            return Err("None is not a valid Commitment Algorithm".into());
        }
        _ => {
            return Err("Unknown is not a valid Commitment Algorithm".into());
        }
    };
    let info = [
        suite.binary_id.as_ref().unwrap().as_ref(),
        KEY_LABEL.as_bytes(),
    ]
    .concat();

    let pseudo_random_key = crypto
        .hkdf_extract()
        .digest_algorithm(digest)
        .salt(message_id.clone())
        .ikm(plaintext_key)
        .send()
        .await
        .unwrap();

    let encrypt_key = crypto
        .hkdf_expand()
        .digest_algorithm(digest)
        .prk(pseudo_random_key.clone())
        .info(info)
        .expected_length(get_kdf_outlen(suite)?)
        .send()
        .await
        .unwrap();

    let commit_key = crypto
        .hkdf_expand()
        .digest_algorithm(digest)
        .prk(pseudo_random_key)
        .info(COMMIT_LABEL.as_bytes())
        .expected_length(commit_len)
        .send()
        .await
        .unwrap();

    Ok(ExpandedKeyMaterial {
        data_key: encrypt_key.into(),
        commitment_key: Some(commit_key.into()),
    })
}

/*
 * Derives key material for encryption/decryption. Delegates out to specific methods
 * based on the input algorithm suite.
 */
pub(crate) async fn derive_keys(
    message_id: &MessageId,
    plaintext_key: &[u8],
    suite: &AlgorithmSuiteInfo,
    crypto: &aws_mpl_rs::aws_cryptography_primitives::client::Client,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    netv4_0_0_retry_policy: types::NetV400RetryPolicy,
    on_net_v4_retry: bool,
) -> Result<ExpandedKeyMaterial, Error> {
    if message_id.is_empty() {
        return Err("Validation Error 12".into());
    }
    if get_encrypt_key_length(suite) as usize != plaintext_key.len() {
        return Err("Validation Error 13".into());
    }

    if suite.message_version.unwrap() == 2 {
        expand_key_material(message_id, plaintext_key, suite, crypto).await
    } else if suite.message_version.unwrap() == 1 {
        let retry =
            netv4_0_0_retry_policy == types::NetV400RetryPolicy::AllowRetry && on_net_v4_retry;
        derive_key(message_id, plaintext_key, suite, crypto, retry).await
    } else {
        Err("Unknown Message Version".into())
    }
}
