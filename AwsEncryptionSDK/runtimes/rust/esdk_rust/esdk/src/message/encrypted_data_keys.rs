// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::serialize_functions::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::EncryptedDataKey;

pub(crate) fn write_edk(w: &mut dyn SafeWrite, edk: &EncryptedDataKey) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-key-entries
    //# Each Encrypted Data Key Entry MUST consist of, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.

    let kp_id_bytes = edk.key_provider_id.as_bytes();
    let Ok(kp_id_len) = u16::try_from(kp_id_bytes.len()) else {
        return ser_err("Key provider ID length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The length of the serialized key provider ID length field MUST be 2 bytes.

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The key provider ID length MUST be interpreted as a UInt16.
    write_u16(w, kp_id_len)?;

    //= specification/data-format/message-header.md#key-provider-id
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.

    //= specification/data-format/message-header.md#key-provider-id
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    write_bytes(w, kp_id_bytes)?;

    let Ok(kp_info_len) = u16::try_from(edk.key_provider_info.len()) else {
        return ser_err("Key provider info length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The length of the serialized key provider information length field MUST be 2 bytes.

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The key provider information length MUST be interpreted as a UInt16.
    write_u16(w, kp_info_len)?;

    //= specification/data-format/message-header.md#key-provider-information
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.

    //= specification/data-format/message-header.md#key-provider-information
    //# The key provider information MUST be interpreted as bytes.
    write_bytes(w, &edk.key_provider_info)?;

    let Ok(edk_len) = u16::try_from(edk.ciphertext.len()) else {
        return ser_err("Encrypted data key length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The encrypted data key length MUST be interpreted as a UInt16.
    write_u16(w, edk_len)?;

    //= specification/data-format/message-header.md#encrypted-data-key
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.

    //= specification/data-format/message-header.md#encrypted-data-key
    //# The encrypted data key MUST be interpreted as bytes.
    write_bytes(w, &edk.ciphertext)
}
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The length of the serialized encrypted data key count MUST be 2 bytes.

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The encrypted data key count MUST be interpreted as a UInt16.
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
    //= specification/client-apis/decrypt.md#parse-the-header
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
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
