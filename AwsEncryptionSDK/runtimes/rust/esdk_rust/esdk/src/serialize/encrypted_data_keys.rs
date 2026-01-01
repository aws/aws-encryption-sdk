// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::serialize_functions::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_rs::types::EncryptedDataKey;

pub(crate) fn write_edk(w: &mut dyn SafeWrite, edk: &EncryptedDataKey) -> Result<(), Error> {
    write_str_u16(w, &edk.key_provider_id)?;
    write_seq_u16(w, &edk.key_provider_info)?;
    write_seq_u16(w, &edk.ciphertext)
}
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    write_u16(w, edks.len() as u16)?;
    for edk in edks {
        write_edk(w, edk)?;
    }
    Ok(())
}

pub(crate) fn read_edk(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<EncryptedDataKey, Error> {
    let provider_id = read_str_u16(r, raw)?;
    let provider_info = read_seq_u16(r, raw)?;
    let ciphertext = read_seq_u16(r, raw)?;
    let edk = EncryptedDataKey::new(provider_id, provider_info, ciphertext);
    Ok(edk)
}
pub(crate) fn read_edks(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<EncryptedDataKey>, Error> {
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
