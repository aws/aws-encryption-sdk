// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Encryption context serialization for message header and AAD.

use super::serializable_types::ESDKCanonicalEncryptionContext;
use super::serialize_functions::{read_str_u16, read_u16, write_bytes, write_u16};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};

/// Read the header's AAD encryption context sub-section.
pub(crate) fn read_canonical_ec(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ESDKCanonicalEncryptionContext, Error> {
    // Empty EC: length 0, no further bytes.
    let bytes = usize::from(read_u16(r, raw)?);
    if bytes == 0 {
        return Ok(Vec::new());
    }

    // Count, then `count` (key, value) pairs.
    let count = usize::from(read_u16(r, raw)?);
    let mut result: ESDKCanonicalEncryptionContext = Vec::with_capacity(count);
    for _ in 0..count {
        let key = read_str_u16(r, raw)?;
        let value = read_str_u16(r, raw)?;
        result.push((key, value));
    }
    Ok(result)
}

/// Write canonical EC bytes for signing/AES-GCM AAD; empty EC writes nothing.
pub(crate) fn write_empty_ec_or_write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    if data.is_empty() {
        //= specification/data-format/message-header.md#key-value-pairs
        //# When the [encryption context](../framework/structures.md#encryption-context) is empty,
        //# this field MUST NOT be included in the [AAD](#aad).
        Ok(())
    } else {
        write_aad(w, data)
    }
}

/// Serialized length of the key-value-pairs body in bytes.
fn get_length(data: &ESDKCanonicalEncryptionContext) -> usize {
    let mut length = 0;
    for pair in data {
        // 2 (key len) + 2 (val len) + key bytes + val bytes.
        length += 4 + pair.0.len() + pair.1.len();
    }
    length
}

/// Write the header's AAD EC sub-section: length + key-value pairs.
pub(crate) fn write_aad_section(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#aad
    //# The AAD MUST consist of, in order,
    //# Key Value Pairs Length,
    //# and Key Value Pairs.
    if data.is_empty() {
        //= specification/data-format/message-header.md#key-value-pairs-length
        //# When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.
        write_u16(w, 0)?;
        return Ok(());
    }

    // Key Value Pairs Length.
    let bytes = get_length(data);

    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The length of the serialized key value pairs length field MUST be 2 bytes.

    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The key value pairs length MUST be interpreted as a UInt16.
    let Ok(bytes_u16) = u16::try_from(bytes) else {
        return ser_err("value too large for u16");
    };
    write_u16(w, bytes_u16)?;

    // Key Value Pairs.
    write_aad(w, data)
}

/// Write the key-value-pairs body: count, then (key, value) pairs.
pub(crate) fn write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    // Count.
    let Ok(data_len) = u16::try_from(data.len()) else {
        return ser_err("value too large for u16");
    };
    write_u16(w, data_len)?;

    for pair in data {
        //= specification/data-format/message-header.md#key-value-pairs
        //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).

        // Key: length + UTF-8 bytes.
        let Ok(key_len) = u16::try_from(pair.0.len()) else {
            return ser_err("value too large for u16");
        };
        write_u16(w, key_len)?;
        write_bytes(w, pair.0.as_bytes())?;

        // Value: length + UTF-8 bytes.
        let Ok(val_len) = u16::try_from(pair.1.len()) else {
            return ser_err("value too large for u16");
        };
        write_u16(w, val_len)?;
        write_bytes(w, pair.1.as_bytes())?;
    }
    Ok(())
}
