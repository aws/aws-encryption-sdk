// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Message footer serialization/deserialization.
//! Maps to data-format/message-footer.md

use super::serialize_functions::{read_seq_u16, write_bytes, write_u16};
use super::*;
use crate::types::{SafeRead, SafeWrite};

/// Write a message footer (signature length + signature bytes).
///
/// The caller is responsible for ensuring this is only called when the algorithm suite
/// includes a signature algorithm.
//= specification/data-format/message.md#structure
//# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
//# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
//# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
//# [message footer](message-footer.md) serialized after the [message body](message-body.md).
pub(crate) fn write_footer(
    w: &mut dyn SafeWrite,
    signature: &[u8],
) -> Result<(), Error> {
    if signature.len() >= u16::MAX.into() {
        return ser_err("Length of signature bytes is larger than the uint16 limit.");
    }
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# This operation MUST then serialize a message footer.
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - [Signature Length](../data-format/message-footer.md#signature-length): The value MUST be the length of the
    //# output of the signature calculation above.
    let len = u16::try_from(signature.len())
        .map_err(|_| Error::from("Sequence length too long for 16 bits"))?;
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# The order for message footer serialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //= specification/data-format/message-footer.md#structure
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.
    //= specification/data-format/message-footer.md#signature-length
    //# The length of the signature length field MUST be 2 bytes.
    //= specification/data-format/message-footer.md#signature-length
    //# The signature length value MUST be a UInt16.
    write_u16(w, len)?;
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# - [Signature](../data-format/message-footer.md#signature): The value MUST be the output of the signature calculation above.
    //= specification/data-format/message-footer.md#signature
    //= type=implication
    //# The signature MUST be interpreted as bytes.
    write_bytes(w, signature)?;
    Ok(())

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=implication
    //# The above serialized bytes MUST NOT be released until the entire message footer has been serialized.
}

/// Read a message footer, returning the signature bytes.
pub(crate) fn read_footer(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# The order for message footer deserialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //= specification/data-format/message-footer.md#structure
    //= reason=read_seq_u16 reads a u16 length (Signature Length) followed by that many bytes (Signature), matching the required order
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.
    //= specification/data-format/message-footer.md#signature
    //= type=implication
    //# The signature MUST be interpreted as bytes.
    read_seq_u16(r, raw)
}
