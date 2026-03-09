// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Message footer serialization/deserialization.
//! Maps to data-format/message-footer.md

use super::serialize_functions::{read_seq_u16, write_seq_u16};
use super::*;
use crate::types::{SafeRead, SafeWrite};

/// Write a message footer (signature length + signature bytes).
//= specification/data-format/message-footer.md#structure
//# The following describes the fields that form the footer.
pub(crate) fn write_footer(
    w: &mut dyn SafeWrite,
    signature: &[u8],
) -> Result<(), Error> {
    if signature.len() >= u16::MAX.into() {
        return ser_err("Length of signature bytes is larger than the uint16 limit.");
    }
    write_seq_u16(w, signature)
}

/// Read a message footer, returning the signature bytes.
pub(crate) fn read_footer(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    read_seq_u16(r, raw)
}
