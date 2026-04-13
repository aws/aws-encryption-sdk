// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::message::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::EncryptedDataKey;

//= aws-encryption-sdk-specification/client-apis/encrypt.md#v1-header
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.

//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
pub(crate) fn write_edks(w: &mut dyn SafeWrite, edks: &[EncryptedDataKey]) -> Result<(), Error> {
    //= specification/data-format/message-header.md#encrypted-data-keys
    //# The Encrypted Data Keys MUST consist of, in order,
    //# Encrypted Data Key Count,
    //# and Encrypted Data Key Entries.

    // Encrypted Data Key Count

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# This value MUST be greater than 0.
    if edks.is_empty() {
        return ser_err("Encrypted data key count must be greater than 0");
    }

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The encrypted data key count MUST be interpreted as a UInt16.
    let Ok(count) = u16::try_from(edks.len()) else {
        return ser_err("Encrypted data key count too large for UInt16");
    };

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The length of the serialized encrypted data key count MUST be 2 bytes.
    write_u16(w, count)?;

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

    let provider_id_bytes = edk.key_provider_id.as_bytes();

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The key provider ID length MUST be interpreted as a UInt16.
    let Ok(provider_id_len) = u16::try_from(provider_id_bytes.len()) else {
        return ser_err("Key provider ID length too large for UInt16");
    };

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    write_u16(w, provider_id_len)?;

    // Key Provider ID

    //= specification/data-format/message-header.md#key-provider-id
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    //= specification/data-format/message-header.md#key-provider-id
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    write_bytes(w, provider_id_bytes)?;

    // Key Provider Information Length

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The key provider information length MUST be interpreted as a UInt16.
    let Ok(provider_info_len) = u16::try_from(edk.key_provider_info.len()) else {
        return ser_err("Key provider information length too large for UInt16");
    };

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The length of the serialized key provider information length field MUST be 2 bytes.
    write_u16(w, provider_info_len)?;

    // Key Provider Information

    //= specification/data-format/message-header.md#key-provider-information
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    //= specification/data-format/message-header.md#key-provider-information
    //# The key provider information MUST be interpreted as bytes.
    write_bytes(w, &edk.key_provider_info)?;

    // Encrypted Data Key Length

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The encrypted data key length MUST be interpreted as a UInt16.
    let Ok(ciphertext_len) = u16::try_from(edk.ciphertext.len()) else {
        return ser_err("Encrypted data key length too large for UInt16");
    };

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.
    write_u16(w, ciphertext_len)?;

    // Encrypted Data Key

    //= specification/data-format/message-header.md#encrypted-data-key
    //= reason=The length field is derived from the same byte slice that is serialized, so they are equal by construction.
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    //= specification/data-format/message-header.md#encrypted-data-key
    //# The encrypted data key MUST be interpreted as bytes.
    write_bytes(w, &edk.ciphertext)?;

    Ok(())
}

//= aws-encryption-sdk-specification/client-apis/decrypt.md#v1-header-deserialization
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be deserialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.

//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be deserialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
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
    //# The encrypted data key count MUST be interpreted as a UInt16.
    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# The length of the serialized encrypted data key count MUST be 2 bytes.
    let count = read_u16(r, raw)?;

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# This value MUST be greater than 0.
    if count == 0 {
        return ser_err("Encrypted data key count must be greater than 0");
    }

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //# This value MUST be less than or equal to the [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys) if the maximum number is configured.
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    if let Some(max) = max_edks {
        if (count as usize) > max.get() {
            return ser_err("Encrypted data key count exceeds maximum");
        }
    }

    // Encrypted Data Key Entries

    let mut edks = Vec::with_capacity(count as usize);
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

    // Key Provider ID Length

    //= specification/data-format/message-header.md#key-provider-id-length
    //# The key provider ID length MUST be interpreted as a UInt16.
    //= specification/data-format/message-header.md#key-provider-id-length
    //# The length of the serialized key provider ID length field MUST be 2 bytes.
    let provider_id_len = read_u16(r, raw)?;

    // Key Provider ID

    //= specification/data-format/message-header.md#key-provider-id
    //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
    let provider_id_bytes = read_vec(r, provider_id_len as usize, raw)?;

    //= specification/data-format/message-header.md#key-provider-id
    //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
    let key_provider_id = String::from_utf8(provider_id_bytes)
        .map_err(|_| Error::from("Key provider ID is not valid UTF-8"))?;

    // Key Provider Information Length

    //= specification/data-format/message-header.md#key-provider-information-length
    //# The key provider information length MUST be interpreted as a UInt16.
    //= specification/data-format/message-header.md#key-provider-information-length
    //# The length of the serialized key provider information length field MUST be 2 bytes.
    let provider_info_len = read_u16(r, raw)?;

    // Key Provider Information

    //= specification/data-format/message-header.md#key-provider-information
    //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
    //= specification/data-format/message-header.md#key-provider-information
    //# The key provider information MUST be interpreted as bytes.
    let key_provider_info = read_vec(r, provider_info_len as usize, raw)?;

    // Encrypted Data Key Length

    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The encrypted data key length MUST be interpreted as a UInt16.
    //= specification/data-format/message-header.md#encrypted-data-key-length
    //# The length of the serialized encrypted data key length field MUST be 2 bytes.
    let ciphertext_len = read_u16(r, raw)?;

    // Encrypted Data Key

    //= specification/data-format/message-header.md#encrypted-data-key
    //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
    //= specification/data-format/message-header.md#encrypted-data-key
    //# The encrypted data key MUST be interpreted as bytes.
    let ciphertext = read_vec(r, ciphertext_len as usize, raw)?;

    Ok(EncryptedDataKey::new(key_provider_id, key_provider_info, ciphertext))
}
