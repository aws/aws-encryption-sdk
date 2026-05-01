// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Encrypted data key serialization and deserialization.

use super::serialize_functions::{read_seq_u16, read_str_u16, read_u16, write_bytes, write_u16};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::EncryptedDataKey;

//= specification/client-apis/encrypt.md#v1-header
//# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
//= specification/client-apis/encrypt.md#v2-header
//# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.

    // Encrypted Data Key Count

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The length of the serialized encrypted data key count MUST be 2 bytes.
    //
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The encrypted data key count MUST be interpreted as a UInt16.
    let Ok(edk_count) = u16::try_from(edks.len()) else {
        return ser_err("Count too large for UInt16");
    };
    write_u16(w, edk_count)?;

    // Encrypted Data Key Entries

    for edk in edks {
        write_edk(w, edk)?;
    }
    Ok(())
}

pub(crate) fn write_edk(w: &mut dyn SafeWrite, edk: &EncryptedDataKey) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-key-entries
    //# Each Encrypted Data Key Entry MUST consist of, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.

    // Key Provider ID Length

    let kp_id_bytes = edk.key_provider_id.as_bytes();

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The key provider ID length MUST be interpreted as a UInt16.
    let Ok(kp_id_len) = u16::try_from(kp_id_bytes.len()) else {
        return ser_err("Key provider ID length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    write_u16(w, kp_id_len)?;

    // Key Provider ID

    //= specification/data-format/message-header.md#key-provider-id
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    //
    //= specification/data-format/message-header.md#key-provider-id
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    write_bytes(w, kp_id_bytes)?;

    // Key Provider Information Length

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The key provider information length MUST be interpreted as a UInt16.
    let Ok(kp_info_len) = u16::try_from(edk.key_provider_info.len()) else {
        return ser_err("Key provider info length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The length of the serialized key provider information length field MUST be 2 bytes.
    write_u16(w, kp_info_len)?;

    // Key Provider Information

    //= specification/data-format/message-header.md#key-provider-information
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    //
    //= specification/data-format/message-header.md#key-provider-information
    //# The key provider information MUST be interpreted as bytes.
    write_bytes(w, &edk.key_provider_info)?;

    // Encrypted Data Key Length

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The encrypted data key length MUST be interpreted as a UInt16.
    let Ok(edk_len) = u16::try_from(edk.ciphertext.len()) else {
        return ser_err("Encrypted data key length too long for 16 bits");
    };

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.
    write_u16(w, edk_len)?;

    // Encrypted Data Key

    //= specification/data-format/message-header.md#encrypted-data-key
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    //
    //= specification/data-format/message-header.md#encrypted-data-key
    //= type=implication
    //# The encrypted data key MUST be interpreted as bytes.
    write_bytes(w, &edk.ciphertext)
}

pub(crate) fn read_edks(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<EncryptedDataKey>, Error> {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.

    // Encrypted Data Key Count

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The length of the serialized encrypted data key count MUST be 2 bytes.
    //
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The encrypted data key count MUST be interpreted as a UInt16.
    let count = read_u16(r, raw)?;

    if let Some(max_edks) = max_edks
        && usize::from(count) > max_edks.get()
    {
        //= specification/client-apis/decrypt.md#v2-header-deserialization
        //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
        //# deserialized from the [message header](../data-format/message-header.md)
        //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
        //# then as soon as that can be determined during deserializing
        //# decrypt MUST process no more bytes and yield an error.
        return ser_err("Ciphertext encrypted data keys exceed maximum encrypted data keys limit");
    }

    // Encrypted Data Key Entries

    let mut edks = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        edks.push(read_edk(r, raw)?);
    }
    Ok(edks)
}

pub(crate) fn read_edk(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<EncryptedDataKey, Error> {
    //= specification/data-format/message-header.md#encrypted-data-key-entries
    //# Each Encrypted Data Key Entry MUST consist of, in order,
    //# Key Provider ID Length,
    //# Key Provider ID,
    //# Key Provider Information Length,
    //# Key Provider Information,
    //# Encrypted Data Key Length,
    //# and Encrypted Data Key.

    // Key Provider ID Length and Key Provider ID

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The key provider ID length MUST be interpreted as a UInt16.
    //
    //= specification/data-format/message-header.md#key-provider-id-length
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    //
    //= specification/data-format/message-header.md#key-provider-id
    //= reason=read_str_u16 reads a u16 length then that many bytes, so the length field and data are equal by construction.
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    //
    //= specification/data-format/message-header.md#key-provider-id
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    let provider_id = read_str_u16(r, raw)?;

    // Key Provider Information Length and Key Provider Information

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The key provider information length MUST be interpreted as a UInt16.
    //
    //= specification/data-format/message-header.md#key-provider-information-length
    //# The length of the serialized key provider information length field MUST be 2 bytes.
    //
    //= specification/data-format/message-header.md#key-provider-information
    //= reason=read_seq_u16 reads a u16 length then that many bytes, so the length field and data are equal by construction.
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    //
    //= specification/data-format/message-header.md#key-provider-information
    //# The key provider information MUST be interpreted as bytes.
    let provider_info = read_seq_u16(r, raw)?;

    // Encrypted Data Key Length and Encrypted Data Key

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The encrypted data key length MUST be interpreted as a UInt16.
    //
    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.
    //
    //= specification/data-format/message-header.md#encrypted-data-key
    //= reason=read_seq_u16 reads a u16 length then that many bytes, so the length field and data are equal by construction.
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    //
    //= specification/data-format/message-header.md#encrypted-data-key
    //= type=implication
    //# The encrypted data key MUST be interpreted as bytes.
    let ciphertext = read_seq_u16(r, raw)?;

    Ok(EncryptedDataKey::new(provider_id, provider_info, ciphertext))
}
