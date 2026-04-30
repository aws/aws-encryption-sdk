// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Message footer serialization/deserialization.
//! Maps to data-format/message-footer.md

use super::serialize_functions::{read_u16, read_vec, write_bytes, write_u16};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};

/// Write a message footer (signature length + signature bytes).
///
/// The caller is responsible for ensuring this is only called when the algorithm suite
/// includes a signature algorithm.
pub(crate) fn write_footer(w: &mut dyn SafeWrite, signature: &[u8]) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# The order for message footer serialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //
    //= specification/data-format/message-footer.md#structure
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.

    // Signature Length

    //= specification/client-apis/encrypt.md#construct-the-signature
    //# The value MUST be the length of the output of the signature calculation above.
    let Ok(len) = u16::try_from(signature.len()) else {
        return ser_err("Sequence length too long for 16 bits");
    };

    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - MUST serialize the [Signature Length](../data-format/message-footer.md#signature-length).
    //
    //= specification/data-format/message-footer.md#signature-length
    //# The length of the signature length field MUST be 2 bytes.
    //
    //= specification/data-format/message-footer.md#signature-length
    //# The signature length value MUST be a UInt16.
    write_u16(w, len)?;

    // Signature

    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - MUST serialize the [Signature](../data-format/message-footer.md#signature).
    //
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# The value MUST be the output of the signature calculation above.
    //
    //= specification/data-format/message-footer.md#signature
    //= type=implication
    //# The signature MUST be interpreted as bytes.
    write_bytes(w, signature)?;

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= reason=write_footer writes length then signature sequentially; Ok(()) is only reached after all writes complete
    //# The above serialized bytes MUST NOT be released until the entire message footer has been serialized.
    Ok(())
}

/// Read a message footer, returning the signature bytes.
pub(crate) fn read_footer(r: &mut dyn SafeRead, raw: &mut dyn SafeWrite) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# The order for message footer deserialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //
    //= specification/data-format/message-footer.md#structure
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.

    // Signature Length

    //= specification/data-format/message-footer.md#signature-length
    //# The length of the signature length field MUST be 2 bytes.
    //
    //= specification/data-format/message-footer.md#signature-length
    //# The signature length value MUST be a UInt16.
    let len = read_u16(r, raw)?;

    // Signature

    //= specification/data-format/message-footer.md#signature
    //= type=implication
    //# The signature MUST be interpreted as bytes.
    read_vec(r, usize::from(len), raw)
}
