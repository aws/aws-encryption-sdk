// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::serializable_types::ESDKCanonicalEncryptionContext;
use super::serialize_functions::{read_str_u16, read_u16, write_bytes, write_u16};
use super::*;
use crate::types::{SafeRead, SafeWrite};

pub(crate) fn read_canonical_ec(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ESDKCanonicalEncryptionContext, Error> {
    let bytes = read_u16(r, raw)? as usize;
    if bytes == 0 {
        return Ok(Vec::new());
    }
    let count = read_u16(r, raw)? as usize;
    let mut result: ESDKCanonicalEncryptionContext = Vec::with_capacity(count);
    for _ in 0..count {
        let key = read_str_u16(r, raw)?;
        let value = read_str_u16(r, raw)?;
        result.push((key, value));
    }
    Ok(result)
}

//= specification/data-format/message-header.md#key-value-pairs
//# When the [encryption context](../framework/structures.md#encryption-context) is empty,
//# this field MUST NOT be included in the [AAD](#aad).
pub(crate) fn write_empty_ec_or_write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    if data.is_empty() {
        Ok(())
    } else {
        write_aad(w, data)
    }
}

fn get_length(data: &ESDKCanonicalEncryptionContext) -> usize {
    let mut length = 0;
    for pair in data {
        length += 4 + pair.0.len() + pair.1.len();
    }
    length
}

//= specification/data-format/message-header.md#aad
//# The AAD MUST be serialized as, in order,
//# Key Value Pairs Length,
//# and Key Value Pairs.
pub(crate) fn write_aad_section(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    if data.is_empty() {
        //= specification/data-format/message-header.md#key-value-pairs-length
        //# When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.
        write_u16(w, 0)?;
        return Ok(());
    }
    let bytes = get_length(data);
    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The length of the serialized key value pairs length field MUST be 2 bytes.
    //= specification/data-format/message-header.md#key-value-pairs-length
    //# The key value pairs length MUST be serialized as a UInt16.
    write_u16(w, bytes as u16)?;
    write_aad(w, data)
}

pub(crate) fn write_aad(
    w: &mut dyn SafeWrite,
    data: &ESDKCanonicalEncryptionContext,
) -> Result<(), Error> {
    write_u16(w, data.len() as u16)?;
    for pair in data {
        //= specification/data-format/message-header.md#key-value-pairs
        //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).
        write_u16(w, pair.0.len() as u16)?;
        write_bytes(w, pair.0.as_bytes())?;
        write_u16(w, pair.1.len() as u16)?;
        write_bytes(w, pair.1.as_bytes())?;
    }
    Ok(())
}
