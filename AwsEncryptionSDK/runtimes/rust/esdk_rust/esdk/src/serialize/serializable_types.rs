// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;

pub(crate) type ESDKEncryptedDataKey = aws_mpl_rs::types::EncryptedDataKey;
pub(crate) type ESDKEncryptedDataKeys = Vec<ESDKEncryptedDataKey>;
pub(crate) type ESDKEncryptionContext = EncryptionContext;
pub(crate) type ESDKEncryptionContextPair = (String, String);
pub(crate) type ESDKCanonicalEncryptionContext = Vec<ESDKEncryptionContextPair>;

const ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH: u64 = u16::MAX as u64 - 2;

pub(crate) const fn get_iv_length(a: &aws_mpl_rs::types::AlgorithmSuiteInfo) -> u8 {
    match a.encrypt.as_ref().unwrap() {
        aws_mpl_rs::types::Encrypt::AesGcm(e) => e.iv_length.unwrap() as u8,
        _ => 0,
    }
}

pub(crate) const fn get_tag_length(a: &aws_mpl_rs::types::AlgorithmSuiteInfo) -> u8 {
    match a.encrypt.as_ref().unwrap() {
        aws_mpl_rs::types::Encrypt::AesGcm(e) => e.tag_length.unwrap() as u8,
        _ => 0,
    }
}

pub(crate) const fn get_encrypt_key_length(a: &aws_mpl_rs::types::AlgorithmSuiteInfo) -> u8 {
    match a.encrypt.as_ref().unwrap() {
        aws_mpl_rs::types::Encrypt::AesGcm(e) => e.key_length.unwrap() as u8,
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

pub(crate) fn is_esdk_encrypted_data_key(edk: &ESDKEncryptedDataKey) -> bool {
    u16::try_from(edk.key_provider_id.as_ref().unwrap().len()).is_ok()
        && u16::try_from(edk.key_provider_info.as_ref().unwrap().as_ref().len()).is_ok()
}

pub(crate) fn is_esdk_encrypted_data_keys(edks: &[ESDKEncryptedDataKey]) -> bool {
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
