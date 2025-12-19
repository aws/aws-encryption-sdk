// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::serializable_types::*;
use super::serialize_functions::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};

pub(crate) fn write_edk(w: &mut dyn SafeWrite, edk: &ESDKEncryptedDataKey) -> Result<(), Error> {
    write_str_u16(w, edk.key_provider_id.as_ref().unwrap())?;
    write_seq_u16(w, edk.key_provider_info.as_ref().unwrap().as_ref())?;
    write_seq_u16(w, edk.ciphertext.as_ref().unwrap().as_ref())
}
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &ESDKEncryptedDataKeys) -> Result<(), Error> {
    write_u16(w, edks.len() as u16)?;
    for edk in edks {
        write_edk(w, edk)?;
    }
    Ok(())
}

pub(crate) fn read_edk(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ESDKEncryptedDataKey, Error> {
    let provider_id = read_str_u16(r, raw)?;
    let provider_info = read_seq_u16(r, raw)?;
    let ciphertext = read_seq_u16(r, raw)?;
    let edk = ESDKEncryptedDataKey::builder()
        .key_provider_id(provider_id)
        .key_provider_info(provider_info)
        .ciphertext(ciphertext)
        .build()?;
    Ok(edk)
}
pub(crate) fn read_edks(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<ESDKEncryptedDataKeys, Error> {
    let count = read_u16(r, raw)?;
    if let Some(max_edks) = max_edks
        && count as usize > max_edks.get()
    {
        return ser_err("Ciphertext encrypted data keys exceed maxEncryptedDataKeys");
    }
    let mut edks = Vec::with_capacity(count as usize);
    for _ in 0..count {
        edks.push(read_edk(r, raw)?);
    }
    Ok(edks)
}
