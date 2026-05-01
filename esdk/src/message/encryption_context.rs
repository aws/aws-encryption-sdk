// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Encryption context serialization for message header and AAD.
//!
//! An encryption context is a canonicalized (sorted, deduplicated) list of
//! UTF-8 key-value pairs. It is serialized in two closely related forms:
//!
//! - The header's AAD field, which wraps the key-value pairs in an outer
//!   `Key Value Pairs Length` (UInt16). See [`write_aad_section`] and
//!   [`read_canonical_ec`].
//! - A bare "canonical" byte stream with no outer length, used as input to
//!   signatures and as AAD for AES-GCM. See [`write_aad`] and
//!   [`write_empty_ec_or_write_aad`].

use super::serializable_types::ESDKCanonicalEncryptionContext;
use super::serialize_functions::{read_str_u16, read_u16, write_bytes, write_u16};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};

/// Read the header's AAD encryption context sub-section and return the
/// canonical (key, value) pairs.
///
/// Reads `Key Value Pairs Length` (UInt16). A length of 0 means the
/// encryption context is empty and nothing further is consumed. Otherwise
/// reads `Key Value Pair Count` (UInt16) followed by that many (key, value)
/// UTF-8 string pairs.
pub(crate) fn read_canonical_ec(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ESDKCanonicalEncryptionContext, Error> {
    // Key Value Pairs Length. When zero, the key-value-pairs sub-field is
    // absent entirely and we're done.
    let bytes = usize::from(read_u16(r, raw)?);
    if bytes == 0 {
        return Ok(Vec::new());
    }

    // Key Value Pair Count, then `count` UTF-8 (key, value) pairs.
    let count = usize::from(read_u16(r, raw)?);
    let mut result: ESDKCanonicalEncryptionContext = Vec::with_capacity(count);
    for _ in 0..count {
        let key = read_str_u16(r, raw)?;
        let value = read_str_u16(r, raw)?;
        result.push((key, value));
    }
    Ok(result)
}

/// Write the canonical encryption context bytes (no outer length prefix) used
/// for signing and as AES-GCM AAD, or write nothing when the context is empty.
///
/// When the encryption context is empty, the spec requires this field to be
/// omitted entirely (not written as a zero-length field).
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

/// Serialized length of the canonical key-value-pairs body, in bytes.
///
/// Each pair contributes two UInt16 length fields (4 bytes total) plus the
/// UTF-8 bytes of the key and value.
fn get_length(data: &ESDKCanonicalEncryptionContext) -> usize {
    let mut length = 0;
    for pair in data {
        // 2 bytes key length + 2 bytes value length + key bytes + value bytes.
        length += 4 + pair.0.len() + pair.1.len();
    }
    length
}

/// Write the header's AAD encryption context sub-section: `Key Value Pairs
/// Length` (UInt16) followed by the canonical key-value-pairs body.
///
/// When the encryption context is empty the length field is written as 0 and
/// the key-value-pairs body is omitted.
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

    // Key Value Pairs Length: total size in bytes of the key-value-pairs body
    // that `write_aad` will emit below.
    let bytes = get_length(data);

    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The length of the serialized key value pairs length field MUST be 2 bytes.

    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The key value pairs length MUST be interpreted as a UInt16.
    let Ok(bytes_u16) = u16::try_from(bytes) else {
        return ser_err("value too large for u16");
    };
    write_u16(w, bytes_u16)?;

    // Key Value Pairs body.
    write_aad(w, data)
}

/// Write the canonical key-value-pairs body with no outer length prefix:
/// `Key Value Pair Count` (UInt16) followed by that many (key, value) pairs.
///
/// Each pair is `Key Length` (UInt16), key UTF-8 bytes, `Value Length`
/// (UInt16), value UTF-8 bytes. Callers use this directly for signature input
/// and AES-GCM AAD, or via [`write_aad_section`] when writing the header AAD.
pub(crate) fn write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    // Key Value Pair Count.
    let Ok(data_len) = u16::try_from(data.len()) else {
        return ser_err("value too large for u16");
    };
    write_u16(w, data_len)?;

    for pair in data {
        //= specification/data-format/message-header.md#key-value-pairs
        //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).

        // Key: length (UInt16) then UTF-8 bytes.
        let Ok(key_len) = u16::try_from(pair.0.len()) else {
            return ser_err("value too large for u16");
        };
        write_u16(w, key_len)?;
        write_bytes(w, pair.0.as_bytes())?;

        // Value: length (UInt16) then UTF-8 bytes.
        let Ok(val_len) = u16::try_from(pair.1.len()) else {
            return ser_err("value too large for u16");
        };
        write_u16(w, val_len)?;
        write_bytes(w, pair.1.as_bytes())?;
    }
    Ok(())
}
