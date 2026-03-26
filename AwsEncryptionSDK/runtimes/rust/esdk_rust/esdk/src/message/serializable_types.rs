// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use aws_mpl_legacy::EncryptedDataKey;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) type ESDKEncryptionContext = EncryptionContext;
pub(crate) type ESDKEncryptionContextPair = (String, String);
pub(crate) type ESDKCanonicalEncryptionContext = Vec<ESDKEncryptionContextPair>;

const ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH: u64 = u16::MAX as u64 - 2;

pub(crate) const fn get_iv_length(a: &AlgorithmSuite) -> u8 {
    match a.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(_e) => 12,
        _ => 0,
    }
}

pub(crate) const fn get_tag_length(a: &AlgorithmSuite) -> u8 {
    match a.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(_e) => 16,
        _ => 0,
    }
}

pub(crate) const fn get_encrypt_key_length(a: &AlgorithmSuite) -> u8 {
    match a.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(e) => e.key_len(),
        _ => 0,
    }
}

/*
 * Length properties of the Encryption Context.
 * The Encryption Context has a complex relationship with length.
 * Each key or value MUST be less than Uint16,
 * However the entire thing MUST also serialize to less than Uint16.
 * In practice, this means than the longest value,
 * given a key of 1 bytes is Uint16-2-2-1.
 * e.g.
 * 2 for the key length
 * 1 for the key data
 * 2 for the value length
 * Uint16-2-2-1 for the value data
 */

pub(crate) fn length(encryption_context: &ESDKEncryptionContext) -> u64 {
    let mut length: usize = 0;
    for (key, value) in encryption_context {
        length += 2 + key.len() + 2 + value.len();
    }
    length as u64
}

pub(crate) fn to_canonical_pairs(
    encryption_context: ESDKEncryptionContext,
) -> ESDKCanonicalEncryptionContext {
    let mut pairs: Vec<(String, String)> = encryption_context.into_iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

pub(crate) fn from_canonical_pairs(pairs: ESDKCanonicalEncryptionContext) -> ESDKEncryptionContext {
    let mut map: ESDKEncryptionContext = ESDKEncryptionContext::new();
    for (key, value) in pairs {
        map.insert(key, value);
    }
    map
}

pub(crate) fn is_esdk_encryption_context(ec: &EncryptionContext) -> bool {
    if ec.len() >= u16::MAX as usize {
        return false;
    }
    if length(ec) >= ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH {
        return false;
    }
    for (key, value) in ec {
        if key.len() >= u16::MAX as usize {
            return false;
        }
        if value.len() >= u16::MAX as usize {
            return false;
        }
    }
    true
}

pub(crate) fn is_esdk_encrypted_data_key(edk: &EncryptedDataKey) -> bool {
    //= aws-encryption-sdk-specification/data-format/message-header.md#key-provider-id-length
    //= type=implication
    //= reason=u16::try_from validates the key provider ID length is representable as UInt16 before serialization
    //# The key provider ID length MUST be serialized as a UInt16.
    u16::try_from(edk.key_provider_id.len()).is_ok()
        && u16::try_from(edk.key_provider_info.len()).is_ok()
}

pub(crate) fn is_esdk_encrypted_data_keys(edks: &[EncryptedDataKey]) -> bool {
    if edks.len() >= u16::MAX as usize {
        return false;
    }
    for edk in edks {
        if !is_esdk_encrypted_data_key(edk) {
            return false;
        }
    }
    true
}
